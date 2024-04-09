use crate::error::CommonError;
use askama::Template;
use warp::{
    http::{response::Response, StatusCode},
    hyper::body::Body,
};

///////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    name: &'a str,
}

pub(crate) async fn process_index() -> Result<warp::reply::Response, CommonError> {
    let index = IndexTemplate { name: "Test name" };

    let output = index.render()?;

    let response = Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(output))?;

    Ok(response)
}
