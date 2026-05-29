# prsq - a persistent todo queue

## Problem from Solana Turbin3 Accelerated Builders' Cohort 2026 Q2
You are building a simple CLI-based Todo application. The app should:
• Allow users to add tasks
• Store tasks in a FIFO queue
• Persist tasks to disk using Borsh serialization
• Restore tasks when the program restarts
• Allow processing (completing) tasks in order
The goal is to design a clean, generic queue system that stores serializable
data and persists it safely. This simulates a very small task management
backend.

## setup
1. clone repo
2. cd prsq
3. cargo build --release

run with `cargo --run` or install globally: `cargo install --path .`

##  usage
- add a task
```bash
prsq add "Buy groceries"
```

- list pending tasks
```bash
prsq list
```

- complete oldest task
```bash
prsq done
```

since tasks are processed in a FIFO order, the oldest task is always completed first. tasks are persisted to `todos.bin` in the working directory using [borsh](https://docs.rs/borsh/1.6.1/borsh/). the file is created on first use and persisted to the disk unless it is overwritten.
