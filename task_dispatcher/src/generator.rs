use crate::task::{Task, TaskKind};


// Generates a list of tasks with alternating CPU and IO types.
pub fn generate_tasks(count: u32) -> Vec<Task> {
    let mut tasks = Vec::new();

    for i in 0..count {
        let kind = if i % 2 == 0 {
            TaskKind::CPU
        } else {
            TaskKind::IO
        };

        let task = Task {
            id: i,
            arrival_time: i as u64,
            kind,
            duration: 100,
        };

        tasks.push(task);
    }

    tasks
}