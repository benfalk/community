use crate::prelude::*;
use axum::{
    routing::{delete, get, post},
    Router,
};
use core_app::Repo;

mod create_household;
mod create_member;
mod delete_household;
mod delete_member;
mod filter_households;
mod update_household;
mod update_member;

pub async fn routes() -> Result<Router> {
    // TODO: Don't use memory db for production silly!
    let repo = Repo::in_memory().await?;

    Ok(Router::new()
        .route("/households", get(filter_households::handler))
        .route("/households", post(create_household::handler))
        .route("/households/:id", post(update_household::handler))
        .route("/households/:id", delete(delete_household::handler))
        .route("/households/:id/members", post(create_member::handler))
        .route("/members/:id", post(update_member::handler))
        .route("/members/:id", delete(delete_household::handler))
        .with_state(repo))
}
