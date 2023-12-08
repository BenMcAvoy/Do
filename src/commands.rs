use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct DoCli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    List,

    Add {
        /// Name of the task
        #[clap(required = true)]
        name: String,

        /// Description of the task
        #[clap(required = true)]
        description: String,

        /// Sets complete
        #[arg(short)]
        completed: bool,
    },
}
