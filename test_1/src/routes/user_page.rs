use crate::{
    context::Context,
    error::CommonError,
    templates::pages::{NotFoundPage, UserPage},
};
use askama::Template;
use uuid::Uuid;
use warp::{
    http::{response::Response, StatusCode},
    hyper::body::Body,
};

///////////////////////////////////////////////////////////////////////////////////////////////

pub(crate) async fn process_user_page(
    user_id: Uuid,
    context: &Context,
) -> Result<warp::reply::Response, CommonError> {
    let output = {
        let users_lock = context.users.lock();

        let get_user_res = users_lock.get(&user_id);

        match get_user_res {
            Some(user) => {
                let index = UserPage { user };
                index.render()?
            }
            None => {
                drop(users_lock);

                let index = NotFoundPage {};
                index.render()?
            }
        }
    };

    let response = Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(output))?;

    Ok(response)
}
