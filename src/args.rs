use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "prsq", version, about = "a persistent todo queue", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// add a new todo item
    Add,
    /// list all pending todo items
    List,
    /// marks an item as completed and removes it
    Done,
}
