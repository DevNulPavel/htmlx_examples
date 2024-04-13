use warp::reject::Reject;

///////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, thiserror::Error)]
pub(crate) enum CommonError {
    #[error("template engine -> {0}")]
    Template(#[from] askama::Error),

    #[error("warp body -> {0}")]
    WarpBody(#[from] warp::http::Error),

    #[error("IO -> {0}")]
    IO(#[from] std::io::Error),

    #[error("serde json -> {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("persist error -> {0}")]
    TempfilePersist(#[from] tempfile::PersistError),

    #[error("cast -> {0}")]
    Cast(#[from] cast::Error),

    #[error("into inner -> {0}")]
    IntoInner(std::io::Error),
}

impl Reject for CommonError {}
