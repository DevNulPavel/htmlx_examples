use crate::{
    context::Context,
    routes::{
        event::{process_new_event, NewEventParams},
        get_user::process_get_user,
        index::process_index,
    },
};
use std::{
    future::Future,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    sync::Arc,
};
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
pub(crate) fn build_warp_server(context: &Arc<Context>) -> (SocketAddr, impl Future<Output = ()>) {
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
    let user = path("user").and(param::<Uuid>()).and(get()).and_then({
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
    let event = path("event")
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

    // Отдача статики скрипта
    let script_data = path("static/htmlx_1.9.11.js").and(get()).map(|| {
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

    // TODO: Добавить условную компрессию при наличии заголовков в запросе

    // Собранные в кучу все роутинги
    let routes = index.or(user).or(event).or(script_data);

    // Адрес сервера для биндинга
    let server_address = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 8080));

    // Стартуем сервер с поддержкой мягкого завершения
    let (server_bind_address, spawned_server_future) = serve(routes)
        .try_bind_with_graceful_shutdown(server_address, async move {
            // Дожидаемся завершения нажатия CTRL-C
            tokio::signal::ctrl_c().await.expect("CTRL-C processing");

            println!("SIGINT signal received");
        })
        .expect("Server spawn problem");

    (server_bind_address, spawned_server_future)
}
