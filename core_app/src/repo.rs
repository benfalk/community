use sqlx::SqlitePool;

mod connect;
mod create_household;
mod update_household;
mod fetch_household;
mod delete_household;
mod create_household_member;
mod delete_household_member;
mod update_household_member;
mod filter_households;

pub struct Repo(SqlitePool);
pub use create_household::CreateHousehold;
pub use update_household::UpdateHousehold;
pub use create_household_member::CreateHouseholdMember;
pub use update_household_member::UpdateHouseholdMember;
