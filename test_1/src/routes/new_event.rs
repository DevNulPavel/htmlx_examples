use crate::{
    context::Context,
    data::event::Event,
    error::CommonError,
    templates::{ErrorPage, NotFoundPage, UserPage},
};
use askama::Template;
use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};
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
    user_uuid: Uuid,
    event_name: NonEmptyString,

    #[serde(deserialize_with = "deserialize_time")]
    event_date: DateTime<Utc>,
}

///////////////////////////////////////////////////////////////////////////////////////////////

pub(crate) async fn process_new_event(
    event_params: NewEventParams,
    context: &Context,
) -> Result<warp::reply::Response, CommonError> {
    // Получаем страничку ответа
    let output = 'output: {
        // Короткая блокировка на юзерах
        let mut users_lock = context.users.lock();

        // Ищем нужного юзера
        let Some(user) = users_lock.get_mut(&event_params.user_uuid) else {
            drop(users_lock);
            let index = NotFoundPage {};
            break 'output index.render()?;
        };

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

                vacant.insert(new_event);

                let user_page = UserPage {
                    user,
                    message: Some("Event inserted"),
                };

                user_page.render()?
            }
            // Почему-то оказался уже такой итем
            Entry::Occupied(_) => {
                let index = ErrorPage {};
                index.render()?
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

    // Будем считать, что время прилетает у нас московское, так что создаем смещение
    let moscow_offset = FixedOffset::west_opt(60 * 60 * 3).unwrap();

    // Сначала создаем время с учетом смещения, конвертим потом в UTC
    let datetime = datetime_naive
        .and_local_timezone(moscow_offset)
        .single()
        .ok_or_else(|| serde::de::Error::custom("Invalid offset"))?
        .to_utc();

    Ok(datetime)
}
