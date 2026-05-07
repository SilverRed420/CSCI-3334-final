# Concurrent Task Dispatcher in Rust

## Overview

This program simulates a task scheduling system using Rust.
It generates tasks, assigns them to queues, and processes them using multiple worker threads.

The system supports different workload types and allows user input to control how tasks are generated.

---

## Requirements

* Rust (latest stable recommended)
* Cargo (comes with Rust)

---

## How to Run

### 1. Open terminal in project folder

Navigate to the project directory:

```bash
cd task_dispatcher
```

---

### 2. Build the project

```bash
cargo build
```

---

### 3. Run the program

```bash
cargo run
```

---

## Program Interaction

When the program starts, you will be asked to choose a workload:

```text
Choose a workload type:
1. Balanced (default)
2. CPU-Intensive
```

Then you will be asked to choose the number of tasks:

```text
Enter number of tasks (default = 500):
```

---

## Workload Types

* **Balanced**

  * Even mix of CPU and IO tasks

* **CPU-Intensive**

  * Mostly CPU tasks (about 80%)

---

## What the Program Does

* Generates tasks with random durations (80–1000 ms)
* Separates tasks into:

  * Long tasks
  * CPU tasks
  * IO tasks
* Uses 12 worker threads:

  * 2 workers prioritize long tasks
  * 10 workers handle CPU/IO tasks
* Workers can help other queues if their own queue is empty
* Tracks performance metrics

---

## Output

The program prints:

* Task processing activity
* Total tasks completed
* Average wait time
* Average turnaround time

---

## Key Features

* Multi-threaded execution
* Multiple scheduling queues
* Dynamic workload selection
* Randomized task durations
* Performance measurement

---

## Notes

* Long tasks are defined as tasks with duration ≥ 700 ms
* Short tasks are prioritized to reduce wait time
* The system uses `Arc<Mutex<...>>` to safely share data between threads

---

## Author

Eleazar Barboza
