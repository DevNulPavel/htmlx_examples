use crate::{context::Context, data::user::User, error::CommonError};
use askama::Template;
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
    let output = {
        let users_lock = context.users.lock();

        let get_user_res = users_lock.get(&user_id);

        match get_user_res {
            Some(user) => {
                let index = GetUserTemplate { user };
                index.render()?
            }
            None => {
                drop(users_lock);

                let index = NotFound {};
                index.render()?
            }
        }
    };

    let response = Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(output))?;

    Ok(response)
}
