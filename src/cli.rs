use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "riki-db")]
#[command(about = "A simple kv storage engine")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Set { key: String, value: String },
    Get { key: String },
    Remove { key: String },
    Prefix { perfix: String },
}
