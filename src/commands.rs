use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct DoCli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    List {
        #[clap(required = true)]
        name: String,
        #[clap(required = true)]
        hash: String,
    },
}
