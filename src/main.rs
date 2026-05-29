use crate::{
    args::{Cli, Commands},
    queue::{Queue, Todo},
};
use chrono::Utc;
use clap::Parser;
use std::{collections::VecDeque, process::Command};

mod args;
mod queue;

fn main() {
    let mut queue: Queue<Todo> = Queue::new();
    let created_at = Utc::now().timestamp() as u64;
    let cli = Cli::parse();
    match cli.command {
        Some(Commands::Add { description }) => queue.enqueue(Todo {
            id: (),
            description,
            created_at,
        }),
    }
}

// add task: todo add "Buy groceries"
// list tasks: todo list
// complete next task: todo done
//
