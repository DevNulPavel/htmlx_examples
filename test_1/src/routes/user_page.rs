use crate::{
    context::Context,
    error::CommonError,
    templates::pages::{NotFoundPage, UserPage},
};
use askama::Template;
use uuid::Uuid;
use warp::{
    http::{response::Response, StatusCode},
    hyper::body::Body,
};

///////////////////////////////////////////////////////////////////////////////////////////////

pub(crate) async fn process_user_page(
    user_id: Uuid,
    context: &Context,
) -> Result<warp::reply::Response, CommonError> {
    let output = {
        // Берем блокировку на пользователе
        let users_lock = context.users.lock();

        // Получаем непосредственно пользователя
        let get_user_res = users_lock.get(&user_id);

        // Если найден, то отдаем его.
        // Если нет, то возвращаем страничку, что не найдено.
        match get_user_res {
            Some(user) => {
                // Создаем шаблон со страничкой юзера
                let index = UserPage { user };
                // Рендерим этот шаблон
                index.render()?
            }
            None => {
                // Удаляем блокировку на пользователе
                drop(users_lock);
                // Возвращаем страничку, что не найдено
                let index = NotFoundPage {};
                // Рендерим этот шаблон
                index.render()?
            }
        }
    };

    // Создаем ответ c кодом 200
    let response = Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(output))?;

    Ok(response)
}
