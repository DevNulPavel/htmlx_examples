use crate::{context::Context, data::user::User, error::CommonError};
use askama::Template;
use std::{
    fs::File,
    io::BufReader,
    sync::{Arc, PoisonError},
};
use warp::{
    http::{response::Response, StatusCode},
    hyper::body::Body,
};

///////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Template)]
#[template(path = "users.html")]
struct IndexTemplate<'a> {
    users: &'a [User],
}

pub(crate) async fn process_index(context: &Context) -> Result<warp::reply::Response, CommonError> {
    let output = {
        let users = context.users.lock();

        let index = IndexTemplate {
            users: users.as_ref(),
        };

        index.render()?
    };

    let response = Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(output))?;

    Ok(response)
}
