use crate::{error::CommonError, templates::pages::UsersPage};
use askama::Template;
use warp::{
    http::{response::Response, StatusCode},
    hyper::body::Body,
};

///////////////////////////////////////////////////////////////////////////////////////////////

/// Обработка отдачи странички со списком юзеров
pub(crate) async fn process_users_page() -> Result<warp::reply::Response, CommonError> {
    // Создаем шаблон
    let index = UsersPage {};

    // Рендерим результат
    // Формируем ответ от сервера
    let output = index.render()?;

    // Ответ от сервера
    let response = Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(output))?;

    Ok(response)
}
