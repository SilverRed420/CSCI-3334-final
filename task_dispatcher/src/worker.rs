use crate::task::Task;
use std::time::{Duration, Instant};

pub fn process_task(mut task: Task, program_start: Instant) -> Task {
    // Simulate task processing by sleeping for the task's duration.
    task.start_time = Some(program_start.elapsed().as_millis());

    std::thread::sleep(Duration::from_millis(task.duration));
    // After processing, set the finish time.
    task.finish_time = Some(program_start.elapsed().as_millis());

    // Return the completed task with updated start and finish times.
    task
}