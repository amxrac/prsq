use anyhow::{Ok, Result};
use prsq::{
    args::{Cli, Commands},
    queue::StoreTodo,
};

use clap::Parser;

fn main() -> Result<()> {
    let mut stored = StoreTodo::load()?;

    let cli = Cli::parse();
    match cli.command {
        Some(Commands::Add { description }) => {
            stored.add(description.clone());
            stored.save()?;
            println!("task: {:?} has added successfully", description);
        }
        Some(Commands::List) => {
            for task in stored.list() {
                println!("{:?}", task);
            }
        }
        Some(Commands::Done) => {
            match stored.done() {
                Some(task) => println!("task: {} has been completed", task.description),
                None => println!("no tasks to complete"),
            }
            stored.save()?;
        }
        None => println!("no command provided. run with --help to see available commands."),
    }
    Ok(())
}
