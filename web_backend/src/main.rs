#![forbid(unsafe_code)]
#![allow(dead_code, unused_imports)]

mod api;
mod error;
mod prelude;
mod utils;

use crate::prelude::*;
use axum::Router;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new().nest("/api", api::routes().await?);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
