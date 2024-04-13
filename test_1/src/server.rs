use crate::{
    context::Context,
    error::CommonError,
    routes::{
        get_user::process_get_user,
        index::process_index,
        new_event::{process_new_event, NewEventParams},
    },
};
use std::{
    future::Future,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    sync::Arc,
};
use tokio_util::sync::CancellationToken;
use uuid::Uuid;
use warp::{
    filters::{
        body::form,
        method::{get, post},
        path::{end, param, path},
    },
    http::{response::Response, status::StatusCode},
    hyper::body::Body,
    reject::Rejection,
    serve, Filter,
};

///////////////////////////////////////////////////////////////////////////////////////////////

/// Создаем и настраиваем сервер
pub(crate) fn build_warp_server(
    context: &Arc<Context>,
    cancellation_grace: CancellationToken,
) -> (SocketAddr, impl Future<Output = ()>) {
    // Роутинг для корневого корня страницы
    let index = end().and(get()).and_then({
        // Клон для лямбды
        let context = context.clone();
        move || {
            // Клон для футуры
            let context = context.clone();
            async move {
                process_index(context.as_ref())
                    .await
                    .map_err(Rejection::from)
            }
        }
    });

    // Роутинг для получения HTML конкретного юзера
    let user = path("user_page").and(param::<Uuid>()).and(get()).and_then({
        // Клон для лямбды
        let context = context.clone();
        move |user_id| {
            // Клон для футуры
            let context = context.clone();
            async move {
                process_get_user(user_id, context.as_ref())
                    .await
                    .map_err(Rejection::from)
            }
        }
    });

    // Обработка ивента
    let event = path("new_event")
        .and(post())
        .and(form::<NewEventParams>())
        .and_then({
            // Клон для лямбды
            let context = context.clone();
            move |event_params| {
                // Клон для футуры
                let context = context.clone();
                async move {
                    process_new_event(event_params, context.as_ref())
                        .await
                        .map_err(Rejection::from)
                }
            }
        });

    // Отдача статики
    let static_data = {
        // Скрипт
        let script_data = path("htmlx_1.9.11.js").and(end()).and(get()).map(|| {
            // Статические данные в бинарнике
            let script_data = include_str!("../static/htmlx_1.9.11.js");

            // Тело
            let body = Body::from(script_data);

            // Сам ответ, можем позволить здесь себе unwrap, так как данные статические
            Response::builder()
                .status(StatusCode::OK)
                .body(body)
                .unwrap()
        });

        // Скрипт
        let style_data = path("style.css").and(end()).and(get()).map(|| {
            // Статические данные в бинарнике
            let script_data = include_str!("../static/style.css");

            // Тело
            let body = Body::from(script_data);

            // Сам ответ, можем позволить здесь себе unwrap, так как данные статические
            Response::builder()
                .status(StatusCode::OK)
                .body(body)
                .unwrap()
        });

        // Общее начало static + другие пути
        path("static").and(script_data.or(style_data))
    };

    // TODO: Добавить условную компрессию при наличии заголовков в запросе

    // Собранные в кучу все роутинги
    let routes = index
        .or(user)
        .or(event)
        .or(static_data)
        .recover(handle_rejection);

    // Адрес сервера для биндинга
    let server_address = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 8080));

    // Стартуем сервер с поддержкой мягкого завершения
    let (server_bind_address, spawned_server_future) = serve(routes)
        .try_bind_with_graceful_shutdown(server_address, async move {
            // Дожидаемся завершения нажатия CTRL-C
            cancellation_grace.cancelled().await;
        })
        .expect("Server spawn problem");

    (server_bind_address, spawned_server_future)
}

///////////////////////////////////////////////////////////////////////////////////////////////

/// Кастомизация обработки различных отвалов в процессе обработки фильтра
pub(super) async fn handle_rejection(
    rej: Rejection,
) -> Result<Response<Body>, std::convert::Infallible> {
    // Нету такого метода
    if rej.is_not_found() {
        // Формируем результат, здесь он у нас точно валидный, так что можно unwrap
        let r = warp::http::Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(warp::hyper::Body::empty())
            .unwrap();

        Ok(r)
    }
    // Конкретная какая-то ошибка
    else if let Some(e) = rej.find::<CommonError>() {
        // Сформируем body ответа, который содержит идентификатор этой самой ошибки в логах
        // Здесь можем делать лишь unwrap
        let body_str = error_json_string(e).unwrap();

        // Формируем результат, здесь он у нас точно валидный, так что можно unwrap
        let r = warp::http::Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(warp::hyper::Body::from(body_str))
            .unwrap();

        Ok(r)
    }
    // Проблема c самим запросом
    else if rej.find::<warp::reject::InvalidHeader>().is_some()
        || rej.find::<warp::reject::InvalidQuery>().is_some()
        || rej.find::<warp::reject::LengthRequired>().is_some()
        || rej.find::<warp::reject::MethodNotAllowed>().is_some()
    {
        // Формируем результат, здесь он у нас точно валидный, так что можно unwrap
        let r = warp::http::Response::builder()
            .status(StatusCode::NOT_IMPLEMENTED)
            .body(warp::hyper::Body::empty())
            .unwrap();

        Ok(r)
    }
    // Все остальное
    else {
        // Формируем результат, здесь он у нас точно валидный, так что можно unwrap
        let r = warp::http::Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(warp::hyper::Body::empty())
            .unwrap();

        Ok(r)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////

fn error_json_string(err: impl std::error::Error) -> Result<String, serde_json::Error> {
    // Сформируем body ответа, который содержит идентификатор этой самой ошибки в логах
    /* let body_str = format_small!(
       128,
       r#"{{ "error_id": "{}", "error": "{}" }}"#,
       error_id.as_u128(),
       e.err
    ); */

    // Локальная структура, чтобы нормально кодировались строки в Json,
    // так как там могут быть всякие символы не особо нужные.
    #[derive(Debug, serde::Serialize)]
    struct ErrResponse<'a> {
        error: &'a str,
    }

    let err_str = format!("{}", err);

    serde_json::to_string(&ErrResponse {
        error: err_str.as_str(),
    })
}
