#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct HouseholdMember {
    pub(crate) id: i64,
    pub(crate) household_id: i64,
    pub(crate) first_name: String,
    pub(crate) last_name: String,
    pub(crate) email: Option<String>,
    pub(crate) cell_number: Option<String>,
}
