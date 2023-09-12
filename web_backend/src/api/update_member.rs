use crate::prelude::*;
use axum::{
    extract::{Path, State},
    Json,
};
use core_app::{
    models::HouseholdMember,
    repo::{Repo, UpdateHouseholdMember},
};

pub async fn handler(
    State(repo): State<Repo>,
    Path(id): Path<i64>,
    Json(mut update): Json<UpdateHouseholdMember>,
) -> Result<Json<HouseholdMember>> {
    update.id = id;
    repo.update_household_member(&update).await?;
    Ok(Json(repo.fetch_household_member(id).await?))
}
