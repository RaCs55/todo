use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "cli")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add { name: String },
    Ls,
    Ct { id: u64 },
    Rm { id: u64 },
}
