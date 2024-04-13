use crate::{
    context::Context, data::event::Event, error::CommonError, helpers::naive_moscow_time_to_utc,
    templates::parts::EventsList,
};
use askama::Template;
use chrono::{DateTime, NaiveDateTime, Utc};
use non_empty_string::NonEmptyString;
use serde::{Deserialize, Deserializer};
use std::{borrow::Cow, collections::btree_map::Entry};
use uuid::Uuid;
use warp::{
    http::{response::Response, StatusCode},
    hyper::body::Body,
};

///////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, serde::Deserialize)]
pub(crate) struct NewEventParams {
    /// Идентификатор юзера, куда мы добавляем данные
    user_uuid: Uuid,

    /// Имя нового события
    event_name: NonEmptyString,

    /// Дата события
    #[serde(deserialize_with = "deserialize_time")]
    event_date: DateTime<Utc>,
}

///////////////////////////////////////////////////////////////////////////////////////////////

pub(crate) async fn process_new_event(
    event_params: NewEventParams,
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

        // Создаем ключ для нового ивента
        let new_event_key = Uuid::now_v7();

        // Бесполезно, но пробуем найти этот ивент, вдруг по непонятной причине у нас будет дубль идентификатора
        match user.events.entry(new_event_key) {
            // Все хорошо, нету такого события
            Entry::Vacant(vacant) => {
                // Создаем новое событие
                let new_event = Event {
                    date: event_params.event_date,
                    name: event_params.event_name,
                    uuid: new_event_key,
                };

                // Добавляем событие
                vacant.insert(new_event);

                // Создаем страничку юзера
                let user_page = EventsList {
                    message: Some("event inserted"),
                    user_uuid: event_params.user_uuid,
                    events: user.events.values(),
                };

                user_page.render()?
            }
            // Почему-то оказался уже такой итем
            Entry::Occupied(_) => {
                return Err(CommonError::InvalidId);
            }
        }
    };

    let response = Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(output))?;

    Ok(response)
}

///////////////////////////////////////////////////////////////////////////////////////////////

fn deserialize_time<'de, D>(d: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    // Парсим просто текст сначала.
    let datetime_str = Cow::<'de, str>::deserialize(d)?;

    // Пробуем теперь распарсить уже конкретно строку
    let datetime_naive = NaiveDateTime::parse_from_str(datetime_str.as_ref(), "%Y-%m-%dT%H:%M")
        .map_err(serde::de::Error::custom)?;

    // Сначала создаем время с учетом смещения, конвертим потом в UTC
    let datetime = naive_moscow_time_to_utc(datetime_naive);

    Ok(datetime)
}
