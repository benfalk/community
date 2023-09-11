use crate::prelude::*;
use super::*;

impl Repo {
    pub async fn delete_household_member(&self, id: i64) -> Result<()> {
        sqlx::query!("
            DELETE FROM household_members
            WHERE id = ?
            RETURNING id
        ",
            id
        )
        .fetch_one(&self.0)
        .await?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::*;
    use super::*;

    #[tokio::test]
    async fn delete() -> Result<()> {
        let repo = Repo::in_memory().await?;
        sqlx::query!("INSERT INTO households (address) VALUES (?)", "OCP Main Building")
            .execute(&repo.0)
            .await?;
        sqlx::query!(
            "
            INSERT INTO household_members
            (
                household_id,
                first_name,
                last_name
            )
            VALUES (?, ?, ?)
        ",
            1,
            "Dick",
            "Jones",
        )
        .execute(&repo.0)
        .await?;
        repo.delete_household_member(1).await?;
        sqlx::query!("SELECT id FROM households WHERE id = 1")
            .fetch_one(&repo.0)
            .await?;
        Ok(())
    }
}
