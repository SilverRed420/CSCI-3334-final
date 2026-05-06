mod task;

use task::{Task, TaskKind};
fn main() {
    let task = Task {
        id: 1,
        arrival_time: 0,
        kind: TaskKind::CPU,
        duration: 100,
    };
    println!("{:?}", task);
}
