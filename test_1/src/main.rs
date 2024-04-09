mod error;
mod routes;

///////////////////////////////////////////////////////////////////////////////////////////////

use crate::error::CommonError;
use askama::Template;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use uuid::Uuid;
use warp::{
    filters::{
        method::{get, post},
        path::{end, path},
    },
    http::{response::Response, StatusCode},
    hyper::body::Body,
    reject::Rejection,
    serve, Filter,
};

///////////////////////////////////////////////////////////////////////////////////////////////

#[tokio::main]
async fn main() {
    // Роутинг для корневого корня страницы
    let index = end()
        .and(get())
        .and_then(|| async { process_index().await.map_err(Rejection::from) });

    // Роутинг для получения HTML конкретного юзера
    let user = path("user")
        .and(warp::path::param::<Uuid>())
        .and(get())
        .and_then(
            |user_id| async move { process_get_user(user_id).await.map_err(Rejection::from) },
        );

    // TODO
    let event = path("event")
        .and(post())
        .and(warp::filters::body::form())
        .and_then(|event_params| async move {
            process_get_user(user_id).await.map_err(Rejection::from)
        });

    // Роутинг для корневого корня страницы
    let clicked = path("clicked")
        .and(post())
        .and_then(|| async { process_clicked().await.map_err(Rejection::from) });

    // Собранные в кучу все роутинги
    let routes = index.or(user).or(clicked);

    // Адрес сервера для биндинга
    let server_address = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 8080));

    // Стартуем сервер с поддержкой мягкого завершения
    let (_server_bind_address, spawned_server_future) = serve(routes)
        .try_bind_with_graceful_shutdown(server_address, async move {
            // Дожидаемся завершения нажатия CTRL-C
            tokio::signal::ctrl_c().await.expect("CTRL-C processing");
        })
        .expect("Server spawn problem");

    // Возвращаем футуру ожидания завершения работы сервера выше
    spawned_server_future.await
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