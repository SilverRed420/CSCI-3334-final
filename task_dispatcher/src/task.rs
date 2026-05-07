#[derive(Debug, Clone)]

// Represents the type of task, either CPU-bound or IO-bound.
pub enum TaskKind {
    CPU,
    IO,
}

#[derive(Debug, Clone)]
// Represents a task with its properties.
pub struct Task {
    pub id: u32,
    pub arrival_time: u64,
    pub kind: TaskKind,
    pub duration: u64,

    pub start_time: Option<u128>,
    pub finish_time: Option<u128>,
}