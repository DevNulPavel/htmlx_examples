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
use data::{event::Event, user::User};
use error::CommonError;
use parking_lot::Mutex;
use std::{
    borrow::Borrow,
    fs::File,
    io::BufReader,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    path::Path,
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

#[tokio::main]
async fn main() {
    // Парсим аргументы приложения
    let args = AppArgs::parse();

    // Создадим из параметров теперь контекст, который мы будем шарить
    let context = {
        // Грузим юзеров
        let users = load_users(&args.users_file_path).expect("users_loading");

        // Создаем контекст общий
        let context = Context {
            users: Mutex::new(users),
        };

        // Arc для потоков
        Arc::new(context)
    };

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

    // Адрес сервера
    println!("Server address: 'http://{}'", server_bind_address);

    // Возвращаем футуру ожидания завершения работы сервера выше
    spawned_server_future.await;

    // Здесь после завершения работы сервера должен остаться лишь один Arc-контекст
    let context = Arc::into_inner(context).expect("context_last_arc");

    // Получаем назад юзеров
    let users = context.users.into_inner();

    // После завершения работы снова сохраняем данные в файлик
    save_users(users, &args.users_file_path).expect("users_save");

    println!("Users saved");
}

///////////////////////////////////////////////////////////////////////////////////////////////

fn load_users(users_file_path: &Path) -> Result<Vec<User>, CommonError> {
    /* // Для этого примера достаточно простого синхронного чтения файлика, вообще без изысков.
    let reader = {
        let file = File::open(args.users_file_path)?;
        BufReader::new(file)
    };

    let users = serde_json::from_reader::<Vec<User>>(reader)?; */

    todo!()
}

fn save_users(users: impl AsRef<[User]>, users_file_path: &Path) -> Result<(), CommonError> {
    todo!()
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
