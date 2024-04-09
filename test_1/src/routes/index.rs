///////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    name: &'a str,
}

async fn process_index() -> Result<warp::reply::Response, CommonError> {
    let index = IndexTemplate { name: "Test name" };

    let output = index.render()?;

    let response = Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(output))?;

    Ok(response)
}
