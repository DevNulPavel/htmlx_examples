use warp::reject::Reject;

///////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, thiserror::Error)]
pub(crate) enum CommonError {
    #[error("template engine -> {0}")]
    Template(#[from] askama::Error),

    #[error("warp body -> {0}")]
    WarpBody(#[from] warp::http::Error),
}

impl Reject for CommonError {}
