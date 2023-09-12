use crate::prelude::*;
use axum::{routing, Router};
use core_app::Repo;

mod create_household;
mod create_member;
mod filter_households;
mod update_household;
mod update_member;

pub async fn routes() -> Result<Router> {
    // TODO: Don't use memory db for production silly!
    let repo = Repo::in_memory().await?;

    Ok(Router::new()
        .route("/households", routing::get(filter_households::handler))
        .route("/households", routing::post(create_household::handler))
        .route("/households/:id", routing::post(update_household::handler))
        .route(
            "/households/:id/members",
            routing::post(create_member::handler),
        )
        .route("/members/:id", routing::post(update_member::handler))
        .with_state(repo))
}
