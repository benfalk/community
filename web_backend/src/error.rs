use axum::{http::StatusCode, response::IntoResponse};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Generic {0}")]
    Generic(String),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    WebServer(#[from] hyper::Error),

    #[error(transparent)]
    App(#[from] core_app::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let err = format!("{self:#?}");
        (StatusCode::INTERNAL_SERVER_ERROR, err).into_response()
    }
}
