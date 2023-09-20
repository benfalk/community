use super::Repo;
use crate::{
    models::{Household, HouseholdMember},
    prelude::*,
    utils::SplitWhileTrait,
};
use sqlx::{sqlite::SqliteRow, QueryBuilder, Row};
use std::num::NonZeroU32;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct FilterHouseholds {
    pub page: Option<NonZeroU32>,
    #[serde(default = "default_page_size")]
    pub per_page: u8,
}

fn default_page_size() -> u8 {
    20
}

impl FilterHouseholds {
    fn limit(&self) -> u8 {
        self.per_page
    }

    fn offset(&self) -> u32 {
        (self.page.map(NonZeroU32::get).unwrap_or(1) - 1) * (self.per_page as u32)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct FilterHouseholdResults {
    pub results: Vec<Household>,
    pub has_next_page: bool,
}

impl Repo {
    pub async fn filter_households(
        &self,
        _ctx: &Context,
        filters: &FilterHouseholds,
    ) -> Result<FilterHouseholdResults> {
        let (mut results, has_next_page) = fetch_households_without_members(self, filters).await?;
        add_members_to_households(self, &mut results).await?;
        Ok(FilterHouseholdResults {
            results,
            has_next_page,
        })
    }
}

async fn fetch_households_without_members(
    repo: &Repo,
    filters: &FilterHouseholds,
) -> Result<(Vec<Household>, bool)> {
    let mut has_next_page = false;
    let mut households = sqlx::query(
        "
        SELECT
            id,
            address
        FROM households
        LIMIT ?
        OFFSET ?
    ",
    )
    .bind(filters.limit() + 1)
    .bind(filters.offset())
    .map(|row: SqliteRow| Household {
        id: row.get("id"),
        address: row.get("address"),
        members: vec![],
    })
    .fetch_all(&repo.0)
    .await?;

    if households.len() > filters.limit() as usize {
        has_next_page = true;
        households.pop();
    }

    Ok((households, has_next_page))
}

async fn add_members_to_households(repo: &Repo, households: &mut Vec<Household>) -> Result<()> {
    if households.is_empty() {
        return Ok(());
    }
    let mut ordered: Vec<_> = households.iter_mut().collect();
    ordered.sort_unstable_by_key(|h| h.id);

    let mut query = QueryBuilder::new(
        "
        SELECT
            id,
            household_id,
            first_name,
            last_name,
            email,
            cell_number
        FROM household_members
        WHERE household_id IN (
    ",
    );
    let mut seperator = query.separated(", ");
    ordered.iter().for_each(|h| {
        seperator.push_bind(h.id);
    });
    query.push(") ORDER BY household_id DESC");

    let mut all_members = query
        .build()
        .map(|row: SqliteRow| HouseholdMember {
            id: row.get("id"),
            household_id: row.get("household_id"),
            first_name: row.get("first_name"),
            last_name: row.get("last_name"),
            email: row.get("email"),
            cell_number: row.get("cell_number"),
        })
        .fetch_all(&repo.0)
        .await?;

    for household in ordered {
        let mut members = all_members.split_while(|m| m.household_id == household.id);
        std::mem::swap(&mut household.members, &mut members);
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::prelude::*;

    #[tokio::test]
    async fn filter() -> Result<()> {
        let ctx = Context::as_root();
        let repo = Repo::in_memory().await?;

        sqlx::query!(
            "INSERT INTO households (address) VALUES (?)",
            "OCP Main Building"
        )
        .execute(&repo.0)
        .await?;

        sqlx::query!(
            "INSERT INTO households (address) VALUES (?)",
            "Detroit Police Office"
        )
        .execute(&repo.0)
        .await?;

        add_member(&repo, 1, "Dick", "Jones").await?;
        add_member(&repo, 2, "Alex", "Murphy").await?;
        add_member(&repo, 1, "Bob", "Morton").await?;
        add_member(&repo, 2, "Anne", "Lewis").await?;

        let filter = FilterHouseholds {
            per_page: 2,
            page: NonZeroU32::new(1),
        };

        let data = repo.filter_households(&ctx, &filter).await?;
        assert_eq!(data.has_next_page, false);
        assert_eq!(data.results.len(), 2);
        assert_eq!(data.results[0].members.len(), 2);
        assert_eq!(data.results[0].members[0].first_name, "Dick");
        assert_eq!(data.results[0].members[1].first_name, "Bob");
        assert_eq!(data.results[1].members.len(), 2);
        assert_eq!(data.results[1].members[0].first_name, "Alex");
        assert_eq!(data.results[1].members[1].first_name, "Anne");
        Ok(())
    }

    async fn add_member(repo: &Repo, household_id: i64, first: &str, last: &str) -> Result<()> {
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
            household_id,
            first,
            last,
        )
        .execute(&repo.0)
        .await?;

        Ok(())
    }
}
