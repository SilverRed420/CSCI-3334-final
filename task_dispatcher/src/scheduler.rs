use std::collections::VecDeque;
use crate::task::Task;
//  Scheduler that manages a queue of tasks
pub struct Scheduler {
    queue: VecDeque<Task>,
}
// Implementation of the Scheduler
impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            queue: VecDeque::new(),
        }
    }
    // Adds a task to the scheduler's queue
    pub fn add_task(&mut self, task: Task) {
        self.queue.push_back(task);
    }
    // Retrieves the next task from the scheduler's queue
    pub fn get_next_task(&mut self) -> Option<Task> {
        self.queue.pop_front()
    }
    // Returns the number of tasks currently in the scheduler's queue
    pub fn queue_len(&self) -> usize {
        self.queue.len()
    }
}