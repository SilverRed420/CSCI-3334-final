use crate::task::{Task, TaskKind};
use rand::Rng;


// Generates a list of tasks with alternating CPU and IO types.
pub fn generate_tasks(count: u32, workload: &str) -> Vec<Task> {
    let mut tasks = Vec::new();
    let mut rng = rand::thread_rng();

    for i in 0..count {
        let kind = match workload {
            "cpu-heavy" => {
                // 80% CPU-bound, 20% IO-bound for CPU-Intensive workload
                if rng.gen_bool(0.8){
                    TaskKind::CPU
                } else {
                    TaskKind::IO
                }
            }
            _ => {
                // Balanced workload: 50% CPU-bound, 50% IO-bound
                if i % 2 == 0 {
                    TaskKind::CPU
                } else {
                    TaskKind::IO
                }
            }
        };
        // Generate a random duration between 80 and 1000 milliseconds for each task.
        let duration = rng.gen_range(80..1001); 

        let task = Task {
            id: i,
            arrival_time: i as u64,
            kind,
            duration,

            start_time: None,
            finish_time: None,
        };

        tasks.push(task);
    }

    tasks
}