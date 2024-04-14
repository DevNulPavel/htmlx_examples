use crate::{
    context::Context,
    error::CommonError,
    routes::{
        events_list::process_events_list,
        new_event::{process_new_event, NewEventParams},
        remove_event::{process_remove_event, RemoveEventParams},
        user_page::process_user_page,
        users_list::process_users_list,
        users_page::process_users_page,
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
    http::{
        header::{HeaderValue, CONTENT_TYPE},
        response::Response,
        status::StatusCode,
    },
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
    let pages = {
        // Роутинг для корневого корня страницы
        let index_page = end()
            .and(get())
            .and_then(|| async { process_users_page().await.map_err(Rejection::from) });

        // Роутинг для получения HTML конкретного юзера
        let user_page = path("user_page").and(param::<Uuid>()).and(get()).and_then({
            // Клон для лямбды
            let context = context.clone();
            move |user_id| {
                // Клон для футуры
                let context = context.clone();
                async move {
                    process_user_page(user_id, context.as_ref())
                        .await
                        .map_err(Rejection::from)
                }
            }
        });

        index_page.or(user_page)
    };

    let parts = {
        // Роутинг для получения HTML конкретного юзера
        let users_list = path("users_list").and(get()).and_then({
            // Клон для лямбды
            let context = context.clone();
            move || {
                // Клон для футуры
                let context = context.clone();
                async move {
                    process_users_list(context.as_ref())
                        .await
                        .map_err(Rejection::from)
                }
            }
        });

        // Роутинг для получения HTML конкретного юзера
        let events_list = path("events_list")
            .and(param::<Uuid>())
            .and(get())
            .and_then({
                // Клон для лямбды
                let context = context.clone();
                move |user_id| {
                    // Клон для футуры
                    let context = context.clone();
                    async move {
                        process_events_list(user_id, context.as_ref())
                            .await
                            .map_err(Rejection::from)
                    }
                }
            });

        // Обработка добавления ивента
        let new_event = path("new_event")
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

        // Обработка удаления ивента
        let remove_event = path("remove_event")
            .and(post())
            .and(form::<RemoveEventParams>())
            .and_then({
                // Клон для лямбды
                let context = context.clone();
                move |event_params| {
                    // Клон для футуры
                    let context = context.clone();
                    async move {
                        process_remove_event(event_params, context.as_ref())
                            .await
                            .map_err(Rejection::from)
                    }
                }
            });

        // Общее начало static + другие пути
        path("parts").and(users_list.or(events_list).or(new_event).or(remove_event))
    };

    // Отдача статики
    let static_data = {
        // Скрипт
        let script_data = path("htmlx_1.9.11.js").and(end()).and(get()).map(|| {
            // Статические данные в бинарнике
            let script_data = include_str!("../static/htmlx_1.9.11.js");
            valid_response_from_static_str(
                script_data,
                mime::APPLICATION_JAVASCRIPT_UTF_8.essence_str(),
            )
        });

        // Скрипт
        let style_data = path("style.css").and(end()).and(get()).map(|| {
            // Статические данные в бинарнике
            let script_data = include_str!("../static/style.css");
            valid_response_from_static_str(script_data, mime::TEXT_CSS_UTF_8.essence_str())
        });

        // Спиннер
        let spinner_data = path("spinner.svg").and(end()).and(get()).map(|| {
            // Статические данные в бинарнике
            let script_data = include_str!("../static/spinner.svg");
            valid_response_from_static_str(script_data, mime::IMAGE_SVG.essence_str())
        });

        // Общее начало static + другие пути
        path("static").and(script_data.or(style_data).or(spinner_data))
    };

    // TODO: Добавить условную компрессию при наличии заголовков в запросе

    // Собранные в кучу все роутинги
    let routes = pages.or(parts).or(static_data).recover(handle_rejection);

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

/// Создаем из статической строки ответ
fn valid_response_from_static_str(
    script_data: &'static str,
    content_type: &'static str,
) -> Response<Body> {
    // Тело
    let body = Body::from(script_data);

    // Сам ответ, можем позволить здесь себе unwrap, так как данные статические
    Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, HeaderValue::from_static(content_type))
        .body(body)
        .unwrap()
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
