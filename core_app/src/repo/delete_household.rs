use super::*;
use crate::prelude::*;

impl Repo {
    pub async fn delete_household(&self, id: i64) -> Result<()> {
        sqlx::query!(
            "
            DELETE FROM households
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
    use super::*;
    use crate::prelude::*;

    #[tokio::test]
    async fn delete() -> Result<()> {
        let repo = Repo::in_memory().await?;
        sqlx::query!(
            "INSERT INTO households (address) VALUES (?)",
            "OCP Main Building"
        )
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
        repo.delete_household(1).await?;
        Ok(())
    }
}
