use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct Task {
    id: isize,
    title: String,
    completed: bool,
}

fn add_task(file: &mut File, tasks: &mut Vec<Task>, id: &mut isize, title: String) -> std::io::Result<()>{
    let task = Task {
        id: *id,
        title: format!("{} {}", title, id),
        completed: false,
    };

    *id += 1;

    let file_row = format!("{:?}\n", task);
    file.write_all(file_row.as_bytes())?;

    tasks.push(task);

    Ok(())
}

fn main() {
    let mut index: isize = 0;
    let mut tasks: Vec<Task> = Vec::new();

    let mut file = File::create("tasks.txt").unwrap();

    add_task(&mut file, &mut tasks, &mut index, "Task".to_string()).unwrap();
    add_task(&mut file, &mut tasks, &mut index, "Task".to_string()).unwrap();
    add_task(&mut file, &mut tasks, &mut index, "Task".to_string()).unwrap();

    println!("{:#?}", tasks);
}
