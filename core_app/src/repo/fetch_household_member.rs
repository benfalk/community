use super::Repo;
use crate::{models::HouseholdMember, prelude::*};
use sqlx::{sqlite::SqliteRow, Row};

impl Repo {
    pub async fn fetch_household_member(&self, _ctx: &Context, id: i64) -> Result<HouseholdMember> {
        let member = sqlx::query(
            "
            SELECT
                id,
                household_id,
                first_name,
                last_name,
                email,
                cell_number
            FROM household_members
            WHERE id = ?
        ",
        )
        .bind(id)
        .map(|row: SqliteRow| HouseholdMember {
            id: row.get("id"),
            household_id: row.get("household_id"),
            first_name: row.get("first_name"),
            last_name: row.get("last_name"),
            email: row.get("email"),
            cell_number: row.get("cell_number"),
        })
        .fetch_one(&self.0)
        .await?;

        Ok(member)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::prelude::*;

    #[tokio::test]
    async fn fetch() -> Result<()> {
        let ctx = Context::as_root();
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

        let member = repo.fetch_household_member(&ctx, 1).await?;
        assert_eq!(member.household_id, 1);
        assert_eq!(member.id, 1);
        assert_eq!(member.first_name, "Dick");
        assert_eq!(member.last_name, "Jones");
        Ok(())
    }
}
