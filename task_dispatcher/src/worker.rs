use crate::task::Task;
use std::thread;
use std::time::Duration;

// Simulates processing a task by sleeping for the task's duration.
pub fn process_task(task: Task) {
    println!("Worker started task {}", task.id);

    thread::sleep(Duration::from_millis(task.duration));

    println!("Worker finished task {} in {} ms", task.id, task.duration);
}