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
    Ok(Json(repo.filter_households(&filters).await?))
}
