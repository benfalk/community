use crate::prelude::*;
use axum::{
    extract::{Query, State},
    Json,
};
use core_app::repo::{FilterHouseholdResults, FilterHouseholds, Repo};

pub async fn handler(
    State(repo): State<Repo>,
    Query(filters): Query<FilterHouseholds>,
) -> Result<Json<FilterHouseholdResults>> {
    let ctx = core_app::Context::as_root();
    Ok(Json(repo.filter_households(&ctx, &filters).await?))
}
