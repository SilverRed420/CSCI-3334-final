mod task;
mod generator;

use generator::generate_tasks;
fn main() {
    let tasks = generate_tasks(10);
    for task in tasks {
        println!("{:?}", task);
    }
}
