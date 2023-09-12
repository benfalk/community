use crate::prelude::*;
use axum::{
    extract::{Path, State},
    Json,
};
use core_app::{
    models::Household,
    repo::{Repo, UpdateHousehold},
};

pub async fn handler(
    State(repo): State<Repo>,
    Path(id): Path<i64>,
    Json(mut update): Json<UpdateHousehold>,
) -> Result<Json<Household>> {
    update.id = id;
    repo.update_household(&update).await?;
    Ok(Json(repo.fetch_household(id).await?))
}
