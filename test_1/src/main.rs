mod args;
mod cancellation;
mod context;
mod data;
mod error;
mod helpers;
mod load_save;
mod routes;
mod server;
mod templates;

///////////////////////////////////////////////////////////////////////////////////////////////

use crate::{
    args::AppArgs,
    cancellation::{spawn_gracefull_shutdown, ShutdownTokens},
    context::Context,
    load_save::{load_users, save_users},
    server::build_warp_server,
};
use clap::Parser;
use parking_lot::Mutex;
use std::{ops::Deref, sync::Arc};

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

    let ShutdownTokens {
        cancelation_grace,
        cancelation_force,
    } = spawn_gracefull_shutdown();

    // Создаем и настраиваем сервер
    let (server_bind_address, spawned_server_future) =
        build_warp_server(&context, cancelation_grace);

    // Адрес сервера
    println!("Server address: 'http://{}'", server_bind_address);

    // Возвращаем футуру ожидания завершения работы сервера выше
    // TODO: Force shutdown support
    tokio::select! {
        _ = spawned_server_future => {
            println!("Server finished");
        }
        _ = cancelation_force.cancelled() => {
            println!("Server FORCE finished");
        }
    }

    // Здесь после завершения работы сервера должен остаться лишь один Arc-контекст,
    // но для принудительного завершения сделаем поддержку еще и сохранения по ссылке
    match Arc::try_unwrap(context) {
        Ok(context) => {
            // Получаем назад юзеров
            let users = context.users.into_inner();

            // После завершения работы снова сохраняем данные в файлик
            save_users(users, &args.users_file_path).expect("users_save");
        }
        Err(context) => {
            let users_lock = context.users.lock();

            // После завершения работы снова сохраняем данные в файлик
            save_users(users_lock.deref(), &args.users_file_path).expect("users_save");
        }
    }

    println!("Users saved");
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
