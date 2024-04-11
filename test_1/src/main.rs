mod args;
mod context;
mod data;
mod error;
mod routes;

///////////////////////////////////////////////////////////////////////////////////////////////

use self::{
    args::AppArgs,
    context::Context,
    routes::{
        event::{process_new_event, NewEventParams},
        get_user::process_get_user,
        index::process_index,
    },
};
use clap::Parser;
use std::{
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
    reject::Rejection,
    serve, Filter,
};

///////////////////////////////////////////////////////////////////////////////////////////////

#[tokio::main]
async fn main() {
    // Парсим аргументы приложения
    let args = AppArgs::parse();

    // Создадим из параметров теперь контекст, который мы будем шарить
    let context = Arc::new(Context {
        events_file_path: args.events_file_path,
        users_file_path: args.users_file_path,
    });

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

    // Собранные в кучу все роутинги
    let routes = index.or(user).or(event);

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

    println!("Server address: 'http://{}'", server_bind_address);

    // Возвращаем футуру ожидания завершения работы сервера выше
    spawned_server_future.await;
}

/* ///////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Template)]
#[template(path = "new_button.html")]
struct NewButton<'a> {
    new_name: &'a str,
}

///////////////////////////////////////////////////////////////////////////////////////////////

async fn process_clicked() -> Result<warp::reply::Response, CommonError> {
    let index = NewButton {
        new_name: "test new name",
    };

    let output = index.render()?;

    let response = Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(output))?;

    Ok(response)
}
 */
