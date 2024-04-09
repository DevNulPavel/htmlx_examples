use crate::error::CommonError;
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
struct EventTemplate<'a> {
    name: &'a str,
}

///////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, serde::Deserialize)]
pub(crate) struct EventParams {
    name: NonEmptyString,
    date: NonEmptyString,
    user_uuid: Uuid,
    uuid: Uuid,
}

///////////////////////////////////////////////////////////////////////////////////////////////

pub(crate) async fn process_event(
    event_params: EventParams,
) -> Result<warp::reply::Response, CommonError> {
    let index = EventTemplate { name: "Test name" };

    let output = index.render()?;

    let response = Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(output))?;

    Ok(response)
}
