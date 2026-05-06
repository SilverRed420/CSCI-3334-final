
mod task;
mod generator;
mod scheduler;
mod worker;
mod metrics;

use std::sync::{Arc, Mutex};
use std::thread;
use metrics::Metrics;
use scheduler::Scheduler;
use generator::generate_tasks;
use worker::process_task;

fn main() {
    let tasks = generate_tasks(10);
    let scheduler = Arc::new(Mutex::new(Scheduler::new()));
    let metrics = Arc::new(Mutex::new(Metrics::new()));

    // add tasks to queue
    {
        let mut sched = scheduler.lock().unwrap();
        for task in tasks {
            sched.add_task(task);
        }
    }

    // create worker threads
    let mut handles = vec![];

    // create 4 workers
    for i in 0..4 {
        // clone Arc references for the scheduler and metrics to move into the thread
        let scheduler_clone = Arc::clone(&scheduler);
        let metrics_clone = Arc::clone(&metrics);

        // spawn a new thread for each worker
        let handle = thread::spawn(move || {
            loop {
                let task_option = {
                    let mut sched = scheduler_clone.lock().unwrap();
                    sched.get_next_task()
                };

                match task_option {
                    Some(task) => {
                        println!("Worker {} picked task {}", i+1, task.id);
                        process_task(task);

                        let mut m = metrics_clone.lock().unwrap();
                        m.complete_task();
                    }
                    None => {
                        break; // no more tasks
                    }
                }
            }
        });

        handles.push(handle);
    }

    // wait for all workers
    for handle in handles {
        handle.join().unwrap();
    }

    let final_metrics = metrics.lock().unwrap();
    println!("Total tasks completed: {}", final_metrics.total_completed);
}
