use std::collections::VecDeque;
use crate::task::{Task, TaskKind};
//  Scheduler that manages a queue of tasks
pub struct Scheduler {
    long_queue: VecDeque<Task>,
    cpu_queue: VecDeque<Task>,
    io_queue: VecDeque<Task>,
    last_was_cpu: bool,
}

// Implementation of the Scheduler
impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            long_queue: VecDeque::new(),
            cpu_queue: VecDeque::new(),
            io_queue: VecDeque::new(),
            last_was_cpu: false,
        }
    }
    // Adds a task to the scheduler's queue
    pub fn add_task(&mut self, task: Task) {
        if task.duration >= 700 {
            self.long_queue.push_back(task);
        } else {
            match task.kind {
                TaskKind::CPU => self.cpu_queue.push_back(task),
                TaskKind::IO => self.io_queue.push_back(task),
            }
        }
    }
    // Retrieves the next task from the scheduler's queue
    pub fn get_next_task(&mut self) -> Option<Task> {
        // Round-robin scheduling between CPU and IO tasks
        self.cpu_queue
            .make_contiguous()
            .sort_by_key(|task| task.duration);

        self.io_queue
            .make_contiguous()
            .sort_by_key(|task| task.duration);

        if !self.last_was_cpu {
            if let Some(task) = self.cpu_queue.pop_front() {
                self.last_was_cpu = true;
                return Some(task)
            }
        }

        if let Some(task) = self.io_queue.pop_front() {
            self.last_was_cpu = false;
            return Some(task)
        }

        if let Some(task) = self.cpu_queue.pop_front() {
            self.last_was_cpu = true;
            return Some(task)
        }

        None
    }

    // Retrieves the next long task from the scheduler's long queue
    pub fn get_next_long_task(&mut self) -> Option<Task> {
        self.long_queue
            .make_contiguous()
            .sort_by_key(|task| task.duration);

        self.long_queue.pop_front()
    }

    // Returns the number of tasks currently in the scheduler's queue
    pub fn queue_len(&self) -> usize {
        self.cpu_queue.len() + self.io_queue.len() + self.long_queue.len()
    }
}