use std::io::{self, prelude::*};

use anyhow::Result;
use colored::Colorize;
use do_cli::Do;

use clap::Parser;
use do_cli::commands::Commands;
use do_cli::commands::DoCli;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
struct Startup {
    username: String,
    password: String,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let mut startup: Startup = confy::load("do", None)?;

    if startup.username.is_empty() || startup.password.is_empty() {
        // Ask the user to input the username and password
        print!("Please enter your username: ");
        io::stdout().flush()?; // Make sure the message is displayed
        let mut username = String::new();
        io::stdin().read_line(&mut username)?;
        username = username.trim().to_string(); // Remove trailing newline

        print!("Please enter your password: ");
        io::stdout().flush()?; // Make sure the message is displayed
        let mut password = String::new();
        io::stdin().read_line(&mut password)?;
        password = password.trim().to_string(); // Remove trailing newline

        // Save the username and password
        startup.username = username;
        startup.password = password;

        // Save the configuration
        confy::store("do", None, &startup)?;
    }

    let config_path = confy::get_configuration_file_path("do", None)?;
    println!("Config file path: {:?}", config_path);

    let do_cli = Do::new(&startup.username, &startup.password).await?;
    let cli = DoCli::parse();

    let result = match cli.command {
        Commands::List => do_cli.list_todos().await,

        Commands::Add {
            name,
            description,
            completed,
        } => todo!(),
    };

    if let Err(e) = result {
        println!("{} {}", "[ ERROR ]".red().bold(), e.to_string().red());
    }

    Ok(())
}
