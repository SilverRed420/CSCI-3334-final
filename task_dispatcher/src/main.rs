
mod task;
mod generator;
mod scheduler;
mod worker;
mod metrics;
mod workload_chooser;

use workload_chooser::{get_workload_choice, get_task_count};
use std::time::Instant;
use std::sync::{Arc, Mutex};
use std::thread;
use metrics::Metrics;
use scheduler::Scheduler;
use generator::generate_tasks;
use worker::process_task;

fn main() {
    let workload = get_workload_choice();
    println!("Selected workload: {}", workload);
    let task_count = get_task_count();
    println!("Number of tasks: {}", task_count);

    let program_start = Instant::now();
    let tasks = generate_tasks(task_count, &workload);
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

    // create 12 workers
    for i in 0..12 {
        // clone Arc references for the scheduler and metrics to move into the thread
        let start_clone = program_start.clone();
        let scheduler_clone = Arc::clone(&scheduler);
        let metrics_clone = Arc::clone(&metrics);

        // spawn a new thread for each worker
        let handle = thread::spawn(move || {
            loop {
                let task_option = {
                    let mut sched = scheduler_clone.lock().unwrap();
                    if i < 2{
                        // For the first two workers, prioritize long tasks
                        if let Some(task) = sched.get_next_long_task() {
                            Some(task)
                        } else {
                            // If no long tasks are available, check for regular tasks
                            sched.get_next_task()
                        }
                    } else {
                        //  For the remaining workers, prioritize regular tasks
                        if let Some(task) = sched.get_next_task(){
                            Some(task)
                        } else {
                            // If no regular tasks are available, check for long tasks
                            sched.get_next_long_task()
                        }
                    }
                };

                match task_option {
                    Some(task) => {
                        println!("Worker {} picked task {} | duration: {}", i, task.id, task.duration);

                        let completed_task = process_task(task, start_clone);

                        let mut m = metrics_clone.lock().unwrap();
                        m.complete_task(&completed_task);
                        println!("Worker {} completed task {} | duration: {}", i, completed_task.id, completed_task.duration);
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
    
    final_metrics.print_summary();
}
