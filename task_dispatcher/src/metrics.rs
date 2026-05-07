use crate::task::Task;

pub struct Metrics {
    pub total_completed: u32,
    pub total_wait_time: u128,
    pub total_turnaround_time: u128,
}

impl Metrics {
    pub fn new() -> Self {
        Metrics {
            total_completed: 0,
            total_wait_time: 0,
            total_turnaround_time: 0,
        }
    }
    // Updates metrics when a task is completed.
    pub fn complete_task(&mut self, task: &Task) {
        self.total_completed += 1;

        if let (Some(start), Some(finish)) = (task.start_time, task.finish_time) {
            let wait = start as i128 - task.arrival_time as i128;
            let turnaround = finish as i128 - task.arrival_time as i128;

            self.total_wait_time += wait.max(0) as u128;
            self.total_turnaround_time += turnaround.max(0) as u128;
        }
    }

    pub fn print_summary(&self) {
        println!("Total tasks: {}", self.total_completed);

        if self.total_completed > 0 {
            println!(
                "Average wait time: {}",
                self.total_wait_time / self.total_completed as u128
            );

            println!(
                "Average turnaround time: {}",
                self.total_turnaround_time / self.total_completed as u128
            );
        }
    }
}