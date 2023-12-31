use crate::prelude::*;
use axum::{
    extract::{Path, State},
    Json,
};
use core_app::{
    models::HouseholdMember,
    repo::{CreateHouseholdMember, Repo},
};

pub async fn handler(
    State(repo): State<Repo>,
    Path(id): Path<i64>,
    Json(mut create): Json<CreateHouseholdMember>,
) -> Result<Json<HouseholdMember>> {
    let ctx = core_app::Context::as_root();
    create.household_id = id;
    Ok(Json(repo.create_household_member(&ctx, &create).await?))
}
