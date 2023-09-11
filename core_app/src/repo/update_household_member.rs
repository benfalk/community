use super::Repo;
use crate::{models::HouseholdMember, prelude::*};
use sqlx::QueryBuilder;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct UpdateHouseholdMember {
    pub id: i64,
    pub household_id: Option<i64>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub cell_number: Option<String>,
}

impl Repo {
    pub async fn update_household_member(
        &self,
        update: &UpdateHouseholdMember,
    ) -> Result<()> {
        let mut builder = QueryBuilder::new("UPDATE household_members SET ");
        let mut separator = builder.separated(", ");

        if let Some(id) = update.household_id {
            separator.push("household_id = ");
            separator.push_bind_unseparated(id);
        }

        if let Some(first_name) = update.first_name.as_ref() {
            separator.push("first_name = ");
            separator.push_bind_unseparated(first_name);
        }

        if let Some(last_name) = update.last_name.as_ref() {
            separator.push("last_name = ");
            separator.push_bind_unseparated(last_name);
        }

        if let Some(email) = update.email.as_ref() {
            if email.is_empty() {
                separator.push("email = NULL");
            } else {
                separator.push("email = ");
                separator.push_bind_unseparated(email);
            }
        }

        if let Some(cell_number) = update.cell_number.as_ref() {
            if cell_number.is_empty() {
                separator.push("cell_number = NULL");
            } else {
                separator.push("cell_number = ");
                separator.push_bind_unseparated(cell_number);
            }
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
    use crate::prelude::*;
    use super::*;

    #[tokio::test]
    async fn update() -> Result<()> {
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

        let update = UpdateHouseholdMember {
            id: 1,
            email: Some("d.jones@ocp.com".to_owned()),
            cell_number: Some("555-5555".to_owned()),
            ..Default::default()
        };

        repo.update_household_member(&update).await?;
        let row = sqlx::query!("SELECT email, cell_number FROM household_members WHERE id = 1").fetch_one(&repo.0).await?;
        assert_eq!(row.email.unwrap(), "d.jones@ocp.com");
        assert_eq!(row.cell_number.unwrap(), "555-5555");
        Ok(())
    }
}
