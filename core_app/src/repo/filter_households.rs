use super::Repo;
use crate::{models::Household, prelude::*};
use std::num::NonZeroU32;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct FilterHouseholds {
    pub page: Option<NonZeroU32>,
    pub per_page: u8,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct FilterHouseholdResults {
    pub results: Vec<Household>,
    pub has_next_page: bool,
}

impl Repo {
    pub async fn filter_households(&self, filters: &FilterHouseholds) -> Result<FilterHouseholds> {
        todo!()
    }
}
