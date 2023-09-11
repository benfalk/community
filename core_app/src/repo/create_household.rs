use super::Repo;
use crate::models::Household;
use crate::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateHousehold {
    pub address: String,
}

impl Repo {
    pub async fn create_household(&self, household: &CreateHousehold) -> Result<Household> {
        let row = sqlx::query!(
            "
            INSERT INTO households
            (
                address
            )
            VALUES (?)
            RETURNING id
        ",
            household.address,
        )
        .fetch_one(&self.0)
        .await?;

        Ok(Household {
            id: row.id,
            address: household.address.to_owned(),
            members: vec![],
        })
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::*;
    use super::*;

    #[tokio::test]
    async fn create() -> Result<()> {
        let repo = Repo::in_memory().await?;
        let create = CreateHousehold {
            address: "6422 Long Lane Drive".to_owned(),
        };
        let household = repo.create_household(&create).await?;
        assert_eq!(household.id, 1);
        assert_eq!(household.address, "6422 Long Lane Drive");
        assert!(household.members.is_empty());
        Ok(())
    }
}
