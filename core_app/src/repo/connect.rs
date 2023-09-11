use crate::prelude::*;
use sqlx::{Sqlite, SqlitePool, migrate::MigrateDatabase};
use super::Repo;

impl Repo {
    pub async fn connect(db_url: &str) -> Result<Self> {
        if !Sqlite::database_exists(db_url).await? {
            Sqlite::create_database(db_url).await?;
        }
        let pool = SqlitePool::connect(db_url).await?;
        sqlx::migrate!().run(&pool).await?;
        Ok(Self(pool))
    }

    #[cfg(test)]
    pub async fn in_memory() -> Result<Self> {
        Self::connect("sqlite://:memory:").await
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::*;
    use super::*;

    #[tokio::test]
    async fn memory_works() -> Result<()> {
        let repo = Repo::in_memory().await?;
        sqlx::query("SELECT * FROM households").fetch_all(&repo.0).await?;
        let result = sqlx::query("SELECT * FROM roflcopterz").fetch_all(&repo.0).await;
        assert!(result.is_err());
        Ok(())
    }
}
