use crate::queue::Todo;
use chrono::Utc;
use std::collections::VecDeque;

mod args;
mod queue;

fn main() {
    // let mut deq = VecDeque::new();
    // deq.push_back(1);
    // deq.push_back(2);
    // deq.push_front(9);
    // println!("{:?}", deq);
    let todo_item = Todo {
        id: 1,
        description: String::from("test"),
        created_at: Utc::now().timestamp() as u64,
    };
}

// add task: todo add "Buy groceries"
// list tasks: todo list
// complete next task: todo done
//
