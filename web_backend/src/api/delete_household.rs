use crate::prelude::*;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use core_app::repo::Repo;

pub async fn handler(State(repo): State<Repo>, Path(id): Path<i64>) -> Result<impl IntoResponse> {
    let ctx = core_app::Context::as_root();
    repo.delete_household(&ctx, id).await?;
    Ok(StatusCode::OK)
}
