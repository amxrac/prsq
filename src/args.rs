use clap::{Parser, Subcommand};

use crate::queue::Todo;

#[derive(Parser)]
#[command(name = "prsq", version, about = "a persistent todo queue", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// add a new todo item
    Add {
        /// task description
        description: String,
    },
    /// list all pending todo items
    List,
    /// marks an item as completed and removes it
    Done,
}
