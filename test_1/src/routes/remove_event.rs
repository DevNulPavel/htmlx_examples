use crate::{context::Context, error::CommonError, templates::parts::EventsList};
use askama::Template;
use uuid::Uuid;
use warp::{
    http::{response::Response, StatusCode},
    hyper::body::Body,
};

///////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, serde::Deserialize)]
pub(crate) struct RemoveEventParams {
    /// Идентификатор юзера для удаления события
    user_uuid: Uuid,

    /// Идентификатор события удаляемого
    event_uuid: Uuid,
}

///////////////////////////////////////////////////////////////////////////////////////////////

pub(crate) async fn process_remove_event(
    event_params: RemoveEventParams,
    context: &Context,
) -> Result<warp::reply::Response, CommonError> {
    // Получаем страничку ответа
    let output = {
        // Короткая блокировка на юзерах
        let mut users_lock = context.users.lock();

        // Ищем нужного юзера
        let user = users_lock
            .get_mut(&event_params.user_uuid)
            .ok_or(CommonError::InvalidId)?;

        // Бесполезно, но пробуем найти этот ивент, вдруг по непонятной причине у нас будет дубль идентификатора
        user.events
            .remove(&event_params.event_uuid)
            .ok_or(CommonError::InvalidId)?;

        // Создаем страничку юзера
        let events_list = EventsList {
            message: Some("Event removed"),
            user_uuid: event_params.user_uuid,
            events: user.events.values(),
        };

        events_list.render()?
    };

    let response = Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(output))?;

    Ok(response)
}
