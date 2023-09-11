use super::HouseholdMember;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Household {
    pub(crate) id: i64,
    pub(crate) address: String,
    pub(crate) members: Vec<HouseholdMember>,
}
