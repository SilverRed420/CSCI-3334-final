#[derive(Debug, Clone)]
pub enum TaskKind {
    CPU,
    IO,
}

#[derive(Debug, Clone)]
pub struct Task {
    pub id: u32,
    pub arrival_time: u64,
    pub kind: TaskKind,
    pub duration: u64,
}