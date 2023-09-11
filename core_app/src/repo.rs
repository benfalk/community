use sqlx::SqlitePool;

mod connect;
mod create_household;
mod create_household_member;
mod delete_household;
mod delete_household_member;
mod fetch_household;
mod filter_households;
mod update_household;
mod update_household_member;

pub struct Repo(SqlitePool);
pub use create_household::CreateHousehold;
pub use create_household_member::CreateHouseholdMember;
pub use filter_households::{FilterHouseholdResults, FilterHouseholds};
pub use update_household::UpdateHousehold;
pub use update_household_member::UpdateHouseholdMember;
