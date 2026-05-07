use std::io;

pub fn get_workload_choice() -> String {
    println!("Choose a workload type:");
    println!("1. Balanced (default)");
    println!("2. CPU-Intensive");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let workload = match input.trim() {
        "2" => "cpu-heavy".to_string(),
        _ => "balanced".to_string(),
    };

    println!("Selected workload: {}", workload);

    workload
}

pub fn get_task_count() -> u32 {
    println!("Enter number of tasks (default = 500):");

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    match input.trim().parse::<u32>() {
        Ok(num) if num > 0 => num,
        _ => 500,
    }
}