
///////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Template)]
#[template(path = "user.html")]
struct GetUserTemplate<'a> {
    name: &'a str,
}

async fn process_get_user(user_id: Uuid) -> Result<warp::reply::Response, CommonError> {
    let index = GetUserTemplate { name: "Test name" };

    let output = index.render()?;

    let response = Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(output))?;

    Ok(response)
}