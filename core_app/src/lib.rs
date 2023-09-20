#![forbid(unsafe_code)]
#![allow(dead_code, unused_imports)]

mod error;
mod prelude;
mod utils;
mod context;

pub mod models;
pub mod repo;
pub use error::Error;
pub use repo::Repo;
pub use context::Context;

#[cfg(test)]
mod test {
    use crate::prelude::*;
    use crate::repo::*;

    #[tokio::test]
    async fn crud_cycle() -> Result<()> {
        let ctx = Context::as_root();
        let repo = Repo::in_memory().await?;
        let create = CreateHousehold {
            address: "OCP Main Building".to_owned(),
        };
        let update = UpdateHousehold {
            id: 1,
            address: Some("Detroit Police Office".to_owned()),
        };
        repo.create_household(&ctx, &create).await?;
        let household = repo.fetch_household(&ctx, 1).await?;
        assert_eq!(household.address, "OCP Main Building");
        repo.update_household(&ctx, &update).await?;
        let household = repo.fetch_household(&ctx, 1).await?;
        assert_eq!(household.id, 1);
        assert_eq!(household.address, "Detroit Police Office");
        repo.delete_household(&ctx, 1).await?;
        Ok(())
    }
}
