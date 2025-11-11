use std::ops::Deref;

use anyhow::Result as AnyResult;
use sqlx::postgres::{PgPool, PgPoolOptions};
use tokio::sync::OnceCell;

pub async fn global_db() -> &'static Database {
    static GLOBAL_DB: OnceCell<Database> = OnceCell::const_new();
    GLOBAL_DB
        .get_or_try_init(Database::try_connect)
        .await
        .expect("failed to connect to db")
}

pub struct Database(PgPool);

impl Database {
    pub async fn try_connect() -> AnyResult<Self> {
        dotenvy::dotenv()?;
        let db_url = std::env::var("DATABASE_URL")?;
        let pool = PgPoolOptions::new().connect(&db_url).await?;
        Ok(Database(pool))
    }
}

impl Deref for Database {
    type Target = PgPool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_db_connection() {
        let db = Database::try_connect().await;
        assert!(db.is_ok());
    }

    #[tokio::test]
    async fn test_global_db() {
        let db = global_db().await;
        assert!(db.acquire().await.is_ok());
    }
}
