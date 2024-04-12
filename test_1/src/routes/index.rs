use crate::{context::Context, data::user::User, error::CommonError};
use askama::Template;
use warp::{
    http::{response::Response, StatusCode},
    hyper::body::Body,
};

///////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Template)]
#[template(path = "users.html")]
struct IndexTemplate<'a> {
    users: &'a [User],
}

/// Обработка отдачи корневой странички
pub(crate) async fn process_index(context: &Context) -> Result<warp::reply::Response, CommonError> {
    // Формируем ответ от сервера
    let output = {
        // Берем короткую блокировку
        let users = context.users.lock();

        dbg!(&users);

        // Создаем шаблон
        let index = IndexTemplate {
            users: users.as_ref(),
        };

        // рендерим результат
        index.render()?
    };

    // Ответ от сервера
    let response = Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(output))?;

    Ok(response)
}
