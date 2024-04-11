use crate::{context::Context, data::user::User, error::CommonError};
use askama::Template;
use std::sync::Arc;
use warp::{
    http::{response::Response, StatusCode},
    hyper::body::Body,
};

///////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Template)]
#[template(path = "users.html")]
struct IndexTemplate {
    users: Vec<User>,
}

pub(crate) async fn process_index(context: &Context) -> Result<warp::reply::Response, CommonError> {
    let users_content = std::fs::read_to_string(context.users_file_path)?;

    // serde_json::fr

    let index = IndexTemplate {};

    let output = index.render()?;

    let response = Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(output))?;

    Ok(response)
}
