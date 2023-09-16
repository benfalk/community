use axum::{
    http::{header, StatusCode, Uri},
    response::{IntoResponse, Response},
    routing, Router,
};
use rust_embed::{EmbeddedFile, RustEmbed};

pub async fn static_handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/').to_string();
    StaticFile(path)
}

#[derive(Debug, RustEmbed)]
#[folder = "../web_frontend/dist"]
struct Assets;

struct StaticFile<T>(pub T);

impl<T> IntoResponse for StaticFile<T>
where
    T: Into<String>,
{
    fn into_response(self) -> Response {
        let path = self.0.into();

        match Assets::get(path.as_str()) {
            Some(content) => {
                // FIXME: Set caching headers where appropriate
                let mime = mime_guess::from_path(path).first_or_octet_stream();
                ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
            }
            None => {
                let index =
                    Assets::get("index.html").expect("index.html file from web_frontend/dist");
                ([(header::CONTENT_TYPE, "text/html")], index.data).into_response()
            }
        }
    }
}
