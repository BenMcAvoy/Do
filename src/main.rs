use anyhow::Result;
use do_cli::Do;

use clap::Parser;
use do_cli::commands::Commands;
use do_cli::commands::DoCli;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let do_cli = Do::new().await?;
    let cli = DoCli::parse();

    match cli.command {
        Commands::List { name, hash } => do_cli.list_todos(name, hash).await?,
    };

    Ok(())
}
