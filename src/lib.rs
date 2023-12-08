#![allow(non_snake_case)]

use anyhow::Result;
use colored::Colorize;
use sqlx::{query, MySqlPool};
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

    pub async fn list_todos(&self, user: String, hash: String) -> Result<()> {
        let users = query!(
            r#"
SELECT UserID, Name, Hash
FROM users
WHERE users.Name = ? AND users.Hash = ?
        "#,
            user,
            hash
        )
        .fetch_all(&self.pool)
        .await?;

        let mut complete_tasks: Vec<String> = Vec::new();
        let mut incomplete_tasks: Vec<String> = Vec::new();

        if let Some(user) = users.get(0) {
            let tasks = query!(
                r#"
SELECT tasks.TaskID, tasks.Name, tasks.Description, tasks.Completed
FROM todos
JOIN users ON todos.UserID = users.UserID
JOIN tasks ON todos.TaskID = tasks.TaskID
WHERE users.UserID = ?;
                "#,
                user.UserID,
            )
            .fetch_all(&self.pool)
            .await?;

            for task in tasks {
                let taskText = format!("- {}: {}", task.Name, &task.Description);

                match task.Completed != 0 {
                    true => complete_tasks.push(taskText),
                    false => incomplete_tasks.push(taskText),
                }
            }
        } else {
            return Ok(());
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
