use sqlx::{migrate::Migrator, sqlite::SqlitePoolOptions, SqlitePool};

pub static MIGRATOR: Migrator = sqlx::migrate!();

#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
}

impl AppState {
    pub async fn new(
        database_url: &str,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        MIGRATOR.run(&pool).await?;

        Ok(Self { pool })
    }
}
