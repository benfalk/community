use super::Repo;
use crate::{models::HouseholdMember, prelude::*};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateHouseholdMember {
    pub household_id: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub cell_number: Option<String>,
}

impl Repo {
    pub async fn create_household_member(
        &self,
        create: &CreateHouseholdMember,
    ) -> Result<HouseholdMember> {
        let row = sqlx::query!(
            "
            INSERT INTO household_members
            (
                household_id,
                first_name,
                last_name,
                email,
                cell_number
            )
            VALUES (?, ?, ?, ?, ?)
            RETURNING id
        ",
            create.household_id,
            create.first_name,
            create.last_name,
            create.email,
            create.cell_number
        )
        .fetch_one(&self.0)
        .await?;

        Ok(HouseholdMember {
            id: row.id,
            household_id: create.household_id,
            first_name: create.first_name.to_owned(),
            last_name: create.last_name.to_owned(),
            email: create.email.to_owned(),
            cell_number: create.cell_number.to_owned(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::prelude::*;

    #[tokio::test]
    async fn create() -> Result<()> {
        let repo = Repo::in_memory().await?;
        sqlx::query!("INSERT INTO households (address) VALUES (?)", "OCP Main Building")
            .execute(&repo.0)
            .await?;
        let create = CreateHouseholdMember {
            household_id: 1,
            first_name: "Alex".to_owned(),
            last_name: "Murphy".to_owned(),
            email: Some("amurphy@detroit-police.org".to_owned()),
            cell_number: Some("911".to_owned()),
        };
        let member = repo.create_household_member(&create).await?;
        assert_eq!(member.id, 1);
        assert_eq!(member.first_name, "Alex");
        assert_eq!(member.last_name, "Murphy");
        assert_eq!(member.email.unwrap(), "amurphy@detroit-police.org");
        assert_eq!(member.cell_number.unwrap(), "911");
        Ok(())
    }
}
