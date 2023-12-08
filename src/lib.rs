use sqlx::{query, MySqlPool};
use colored::Colorize;
use anyhow::Result;
use std::env;

pub mod commands;

pub struct Do {
    pool: MySqlPool,
}

impl Do {
    pub async fn new() -> Result<Self> {
        let pool = MySqlPool::connect(&env::var("DATABASE_URL")?).await?;

        Ok(Self { pool })
    }

    pub async fn list_todos(&self) -> Result<()> {
        let records = query!(
            r#"
SELECT TodoID Description, Completed
FROM todos
ORDER BY id ASC;
        "#
        )
        .fetch_all(&self.pool)
        .await?;

        let mut complete_tasks = Vec::new();
        let mut incomplete_tasks = Vec::new();

        for record in records {
            let task = format!("- {}: {}", record.id, &record.description);
            if record.done {
                complete_tasks.push(task);
            } else {
                incomplete_tasks.push(task);
            }
        }

        if !complete_tasks.is_empty() {
            println!("{}", "Complete tasks:".green().bold());
            for task in complete_tasks {
                println!("  {}", task.green());
            }
        }

        if !incomplete_tasks.is_empty() {
            println!("{}", "Incomplete tasks:".red().bold());
            for task in incomplete_tasks {
                println!("  {}", task.red());
            }
        }

        Ok(())
    }
}
