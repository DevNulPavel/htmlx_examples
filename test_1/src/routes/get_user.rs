use crate::{context::Context, data::user::User, error::CommonError};
use askama::Template;
use non_empty_string::NonEmptyString;
use uuid::Uuid;
use warp::{
    http::{response::Response, StatusCode},
    hyper::body::Body,
};

///////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Template)]
#[template(path = "user.html")]
struct GetUserTemplate<'a> {
    user: &'a User,
}

///////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Template)]
#[template(path = "not_found.html")]
struct NotFound {}

///////////////////////////////////////////////////////////////////////////////////////////////

pub(crate) async fn process_get_user(
    user_id: Uuid,
    context: &Context,
) -> Result<warp::reply::Response, CommonError> {
    {
        let users = context.users.lock();

        users.get(value)
    }
    let index = GetUserTemplate {
        user: User {
            name: NonEmptyString::new(String::from("test")).unwrap(),
            uuid: Uuid::new_v4(),
            events: Vec::new(),
            // name: NonEmptyString::new(String::from("test")).unwrap(),
        },
    };

    let output = index.render()?;

    let response = Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(output))?;

    Ok(response)
}
