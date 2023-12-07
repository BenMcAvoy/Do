use sqlx::{Pool, Sqlite, SqlitePool};
use anyhow::Result;
use std::env;

pub struct Do {
    pool: Pool<Sqlite>,
}

impl Do {
    pub async fn new() -> Result<Self> {
        let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;

        Ok(Self { pool })
    }
}
