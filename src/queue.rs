use anyhow::Result;
use chrono::Utc;
use std::{
    collections::VecDeque,
    fs,
    io::{BufReader, BufWriter},
    path::Path,
};

use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct Todo {
    pub id: u64,
    pub description: String,
    pub created_at: u64,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct Queue<T> {
    items: VecDeque<T>,
}

#[allow(dead_code)]
impl<T> Queue<T> {
    pub fn new() -> Self {
        Self {
            items: VecDeque::new(),
        }
    }

    pub fn enqueue(&mut self, todo_item: T) {
        self.items.push_back(todo_item);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.items.pop_front()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn peek(&self) -> Option<&T> {
        self.items.front()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.items.iter()
    }
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct StoreTodo {
    pub queue: Queue<Todo>,
    pub last_id: u64,
}

impl StoreTodo {
    pub fn new() -> Self {
        Self {
            queue: Queue::new(),
            last_id: 0,
        }
    }

    pub fn add(&mut self, description: String) {
        self.last_id += 1;
        self.queue.enqueue(Todo {
            id: self.last_id,
            description,
            created_at: Utc::now().timestamp() as u64,
        });
    }

    pub fn list(&self) -> impl Iterator<Item = &Todo> {
        self.queue.iter()
    }

    pub fn done(&mut self) -> Option<Todo> {
        self.queue.dequeue()
    }

    pub fn save(&self) -> Result<()> {
        let file = fs::File::create("todos.bin")?;
        let mut writer = BufWriter::new(file);
        borsh::to_writer(&mut writer, &self)?;
        Ok(())
    }

    pub fn load() -> Result<Self> {
        if Path::new("todos.bin").exists() {
            let file = fs::File::open("todos.bin")?;
            let mut reader = BufReader::new(file);
            let data = borsh::from_reader(&mut reader)?;
            Ok(data)
        } else {
            Ok(Self::new())
        }
    }
}
