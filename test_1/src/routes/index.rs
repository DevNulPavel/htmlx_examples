use crate::{context::Context, error::CommonError, templates::IndexPage};
use askama::Template;
use warp::{
    http::{response::Response, StatusCode},
    hyper::body::Body,
};

///////////////////////////////////////////////////////////////////////////////////////////////

/// Обработка отдачи корневой странички
pub(crate) async fn process_index(context: &Context) -> Result<warp::reply::Response, CommonError> {
    // Формируем ответ от сервера
    let output = {
        // Берем короткую блокировку
        let users = context.users.lock();

        // Создаем шаблон
        let index = IndexPage {
            users: users.values(),
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
