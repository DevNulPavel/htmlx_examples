use crate::{data::event::Event, error::CommonError};
use askama::Template;
use non_empty_string::NonEmptyString;
use uuid::Uuid;
use warp::{
    http::{response::Response, StatusCode},
    hyper::body::Body,
};

///////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Template)]
#[template(path = "events.html")]
struct EventsTemplate {
    events: Vec<Event>,
}

///////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, serde::Deserialize)]
pub(crate) struct NewEventParams {
    event_uuid: Uuid,
    name: NonEmptyString,
    date: NonEmptyString,
    user_uuid: Uuid,
}

///////////////////////////////////////////////////////////////////////////////////////////////

pub(crate) async fn process_event(
    event_params: NewEventParams,
) -> Result<warp::reply::Response, CommonError> {
    let index = EventsTemplate { events: Vec::new() };

    let output = index.render()?;

    let response = Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(output))?;

    Ok(response)
}
