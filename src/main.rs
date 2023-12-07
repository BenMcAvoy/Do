use anyhow::Result;
use do_cli::Do;

use do_cli::commands::Commands;
use do_cli::commands::DoCli;
use clap::Parser;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let do_cli = Do::new().await?;
    let cli = DoCli::parse();

    match cli.command {
        Commands::List => do_cli.list_todos().await?,
    };

    Ok(())
}
