///////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Template)]
#[template(path = "event.html")]
struct EventTemplate<'a> {
    name: &'a str,
}

#[derive(Debug, serde::Deserialize)]
struct EventParams {}

async fn process_event(event_params: EventParams) -> Result<warp::reply::Response, CommonError> {
    let index = EventTemplate { name: "Test name" };

    let output = index.render()?;

    let response = Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(output))?;

    Ok(response)
}
