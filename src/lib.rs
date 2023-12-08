#![allow(non_snake_case)]

use anyhow::Result;
use colored::Colorize;
use sqlx::{query, MySqlPool};
use std::env;

use crate::errors::DoError;

pub mod commands;
pub mod errors;

pub struct Do {
    pool: MySqlPool,

    userID: i32,
}

impl Do {
    pub async fn new(name: &str, hash: &str) -> Result<Self> {
        let pool = MySqlPool::connect(&env::var("DATABASE_URL")?).await?;

        let userID;

        let users = query!(
            r#"
SELECT UserID, Name, Hash
FROM users
WHERE users.Name = ? AND users.Hash = ?
        "#,
            name,
            hash
        )
        .fetch_all(&pool)
        .await?;

        if let Some(user) = users.get(0) {
            userID = user.UserID;
        } else {
            return Err(DoError::InvalidLogin.into());
        }

        Ok(Self { pool, userID })
    }

    pub async fn list_todos(&self) -> Result<()> {
        let tasks = query!(
            r#"
SELECT tasks.TaskID, tasks.Name, tasks.Description, tasks.Completed
FROM todos
JOIN users ON todos.UserID = users.UserID
JOIN tasks ON todos.TaskID = tasks.TaskID
WHERE users.UserID = ?;
                "#,
            self.userID,
        )
        .fetch_all(&self.pool)
        .await?;

        let mut complete_tasks = Vec::new();
        let mut incomplete_tasks = Vec::new();

        for task in tasks {
            match task.Completed != 0 {
                true => complete_tasks.push(task),
                false => incomplete_tasks.push(task),
            }
        }

        if !complete_tasks.is_empty() {
            println!("{}", "Complete tasks:".green().bold());
            for task in complete_tasks {
                println!("  {} - {}", task.Name.green(), task.Description.green());
            }
        }

        if !incomplete_tasks.is_empty() {
            println!("{}", "Incomplete tasks:".red().bold());
            for task in incomplete_tasks {
                println!("  {} - {}", task.Name.red(), task.Description.red());
            }
        }

        Ok(())
    }
}
