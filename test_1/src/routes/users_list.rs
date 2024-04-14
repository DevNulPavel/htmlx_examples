use crate::{context::Context, error::CommonError, templates::parts::UsersList};
use askama::Template;
use warp::{
    http::{response::Response, StatusCode},
    hyper::body::Body,
};

///////////////////////////////////////////////////////////////////////////////////////////////

pub(crate) async fn process_users_list(
    context: &Context,
) -> Result<warp::reply::Response, CommonError> {
    let output = {
        // Получаем блокировку юзеров из контекста
        let users_lock = context.users.lock();

        // Создаем шаблон
        let index = UsersList {
            users: users_lock.values(),
        };

        // рендерим результат
        index.render()?
    };

    // Создаем ответ с кодом 200
    let response = Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(output))?;

    Ok(response)
}

///////////////////////////////////////////////////////////////////////////////////////////////

// Тест для шаблона списка пользователей
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        data::{event::Event, user::User},
        templates::parts::EventsList,
    };
    use chrono::Utc;
    use std::collections::BTreeMap;
    use uuid::Uuid;

    // Проверяем, что шаблон списка пользователей генерируется верный
    #[test]
    fn test_template_list_users() {
        let user1 = User {
            uuid: Uuid::now_v7(),
            name: "Test".to_string().try_into().unwrap(),
            events: BTreeMap::new(),
        };

        let users = [user1];

        let users_template = UsersList {
            users: users.iter(),
        };

        let users_page = users_template.render().unwrap();

        assert_ne!(users_page, "");
    }

    #[test]
    fn test_template_list_events() {
        let event1 = Event {
            uuid: Uuid::now_v7(),
            name: "Test".to_string().try_into().unwrap(),
            date: Utc::now(),
        };

        let events = [event1];

        let events_template = EventsList {
            events: events.iter(),
            message: None,
            user_uuid: Uuid::now_v7(),
        };

        let events_page = events_template.render().unwrap();

        assert_ne!(events_page, "");
    }

    #[test]
    fn test_events_list_with_message() {
        let event1 = Event {
            uuid: Uuid::now_v7(),
            name: "Test".to_string().try_into().unwrap(),
            date: Utc::now(),
        };

        let events = [event1];

        let events_template = EventsList {
            events: events.iter(),
            message: Some("Test"),
            user_uuid: Uuid::now_v7(),
        };

        let events_page = events_template.render().unwrap();

        assert_eq!(events_page, "Test");
    }
}
