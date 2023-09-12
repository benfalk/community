use super::Repo;
use crate::prelude::*;
use sqlx::QueryBuilder;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UpdateHousehold {
    #[serde(default = "Default::default")]
    pub id: i64,
    pub address: Option<String>,
}

impl Repo {
    pub async fn update_household(&self, update: &UpdateHousehold) -> Result<()> {
        let mut builder = QueryBuilder::new("UPDATE households SET ");
        let mut separator = builder.separated(", ");

        if let Some(address) = update.address.as_ref() {
            separator.push("address = ");
            separator.push_bind_unseparated(address);
        }

        builder.push(" WHERE id = ");
        builder.push_bind(update.id);
        builder.push(" RETURNING id");
        builder.build().fetch_one(&self.0).await?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::prelude::*;

    #[tokio::test]
    async fn update() -> Result<()> {
        let repo = Repo::in_memory().await?;
        let address = "Sour Lane".to_owned();
        sqlx::query!("INSERT INTO households (address) VALUES (?)", "Rofl Lane")
            .execute(&repo.0)
            .await?;
        let update = UpdateHousehold {
            id: 1,
            address: Some(address.clone()),
        };
        repo.update_household(&update).await?;
        sqlx::query!(
            "
            SELECT id
            FROM households
            WHERE id = ?
            AND address = ?
        ",
            1,
            address,
        )
        .fetch_one(&repo.0)
        .await?;
        Ok(())
    }
}
