use sqlx::SqlitePool;
use anyhow::Result;
use std::env;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    // Create a connection pool for our database
    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;

    println!("Initialised pool");

    Ok(())
}
