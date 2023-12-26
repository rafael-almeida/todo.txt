#[derive(Debug)]
struct Task {
    id: isize,
    title: String,
    completed: bool,
}

fn add_task(tasks: &mut Vec<Task>, id: &mut isize, title: String) {
    let task = Task {
        id: *id,
        title: format!("{title} {}", id.to_string()),
        completed: false,
    };

    *id += 1;
    tasks.push(task);
}

fn main() {
    let mut index: isize = 0;
    let mut tasks: Vec<Task> = Vec::new();
    add_task(&mut tasks, &mut index, "Task".to_string());
    add_task(&mut tasks, &mut index, "Task".to_string());
    add_task(&mut tasks, &mut index, "Task".to_string());
    println!("{:#?}", tasks);
}
