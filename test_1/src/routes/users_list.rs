use crate::{context::Context, error::CommonError, templates::parts::UsersList};
use askama::Template;
use warp::{
    http::{response::Response, StatusCode},
    hyper::body::Body,
};

///////////////////////////////////////////////////////////////////////////////////////////////

pub(crate) async fn process_users_list(
    context: &Context,
) -> Result<warp::reply::Response, CommonError> {
    let output = {
        let users_lock = context.users.lock();

        // Создаем шаблон
        let index = UsersList {
            users: users_lock.values(),
        };

        // рендерим результат
        index.render()?
    };

    let response = Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(output))?;

    Ok(response)
}
