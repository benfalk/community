use crate::prelude::*;
use axum::{extract::State, Json};
use core_app::{
    models::Household,
    repo::{CreateHousehold, Repo},
};

pub async fn handler(
    State(repo): State<Repo>,
    Json(create): Json<CreateHousehold>,
) -> Result<Json<Household>> {
    Ok(Json(repo.create_household(&create).await?))
}
