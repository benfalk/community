use super::Repo;
use crate::{
    models::{Household, HouseholdMember},
    prelude::*,
};
use sqlx::{sqlite::SqliteRow, Row};

impl Repo {
    pub async fn fetch_household(&self, id: i64) -> Result<Household> {
        let household_fetch = sqlx::query(
            "
            SELECT
                id,
                address
            FROM households
            WHERE id = ?
        ",
        )
        .bind(id)
        .map(|row: SqliteRow| Household {
            id: row.get("id"),
            address: row.get("address"),
            members: vec![],
        })
        .fetch_one(&self.0);

        let members_fetch = sqlx::query(
            "
            SELECT
                id,
                household_id,
                first_name,
                last_name,
                email,
                cell_number
            FROM household_members
            WHERE household_id = ?
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
        .fetch_all(&self.0);

        let (household_result, members_result) = tokio::join!(household_fetch, members_fetch);
        let mut household = household_result?;
        let mut members = members_result?;
        std::mem::swap(&mut household.members, &mut members);
        Ok(household)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn fetch() -> Result<()> {
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

        let household = repo.fetch_household(1).await?;
        assert_eq!(household.id, 1);
        assert_eq!(household.address, "OCP Main Building");
        assert_eq!(household.members.len(), 1);
        assert_eq!(household.members[0].household_id, 1);
        assert_eq!(household.members[0].first_name, "Dick");
        assert_eq!(household.members[0].last_name, "Jones");
        assert!(household.members[0].email.is_none());
        assert!(household.members[0].cell_number.is_none());
        Ok(())
    }
}
