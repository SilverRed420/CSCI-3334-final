pub struct Metrics {
    pub total_completed: u32,
}// Metrics struct to track the total number of completed tasks

impl Metrics {
    // Creates a new Metrics instance with total_completed initialized to 0
    pub fn new() -> Self {
        Metrics {
            total_completed: 0,
        }
    }

    // Increments the total_completed count by 1
    pub fn complete_task(&mut self) {
        self.total_completed += 1;
    }
}