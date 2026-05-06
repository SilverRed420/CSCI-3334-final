mod task;
mod generator;
mod scheduler;
mod worker;

use scheduler::Scheduler;
use generator::generate_tasks;
use worker::process_task;

fn main() {
    let tasks = generate_tasks(10);
    let mut scheduler = Scheduler::new();
    // Add generated tasks to the scheduler's queue
    for task in tasks {
        scheduler.add_task(task);
    }
    // Print the number of tasks currently in the scheduler's queue
    println!("Tasks in current queue: {}", scheduler.queue_len());
    // Dispatch tasks from the scheduler's queue and print their details
    while let Some(task) = scheduler.get_next_task() {
        process_task(task);
    }
}
