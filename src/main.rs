use std::fs::{File, OpenOptions};
use std::io::{self, prelude::*};
use std::num::ParseIntError;
use std::str::{FromStr, ParseBoolError};

#[derive(Debug)]
struct Task {
    id: isize,
    title: String,
    completed: bool,
}

enum ParseTaskError {
    IntError(ParseIntError),
    BoolError(ParseBoolError),
}

macro_rules! from_error {
    ($from:ty, $to:ident) => {
        impl From<$from> for ParseTaskError {
            fn from(err: $from) -> Self {
                ParseTaskError::$to(err)
            }
        }
    };
}

from_error!(ParseIntError, IntError);
from_error!(ParseBoolError, BoolError);

impl FromStr for Task {
    type Err = ParseTaskError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(",").collect();

        let id = parts[0].parse::<isize>()?;
        let title = parts[1].to_string();
        let completed = parts[2].parse::<bool>()?;

        Ok(Task {
            id,
            title,
            completed,
        })
    }
}

fn read_tasks(file: &mut File) -> std::io::Result<Vec<Task>> {
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    file.seek(io::SeekFrom::Start(0))?;

    let tasks = contents
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect::<Vec<Task>>();

    Ok(tasks)
}

fn write_tasks(file: &mut File, tasks: &mut Vec<Task>) -> std::io::Result<()> {
    file.set_len(0)?;
    file.seek(io::SeekFrom::Start(0))?;

    for task in tasks {
        let file_row = format!("{},{},{}\n", task.id, task.title, task.completed);
        file.write_all(file_row.as_bytes())?;
    }

    Ok(())
}

fn add_task(tasks: &mut Vec<Task>, title: String) {
    let mut id: isize = 0;

    if tasks.len() > 0 {
        id = tasks[tasks.len() - 1].id + 1;
    }

    let task = Task {
        id,
        title: format!("{} {}", title, id),
        completed: false,
    };

    tasks.push(task);
}

fn toggle_task(tasks: &mut Vec<Task>, id: isize) {
    for task in tasks {
        if task.id == id {
            task.completed = !task.completed;
        }
    }
}

fn display_tasks(tasks: &[Task]) {
    let mut rows_len: [usize; 3] = [0, 0, 0];

    for task in tasks {
        rows_len[0] = rows_len[0].max(task.id.to_string().len());
        rows_len[1] = rows_len[1].max(task.title.to_string().len());
        rows_len[2] = rows_len[2].max(task.completed.to_string().len());
    }

    let a = rows_len[0];
    let b = rows_len[1];
    let c = rows_len[2];
    let w = a + b + c + 10;

    println!("{:->w$}", "-");
    println!("{:^w$}", "Tasks");
    println!("{:->w$}", "-");

    for task in tasks {
        println!(
            "| {:a$} | {:b$} | {:c$} |",
            task.id,
            task.title,
            task.completed.to_string()
        );
    }
}

fn main() -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .read(true)
        .append(true)
        .open("data.txt")?;

    let mut tasks = read_tasks(&mut file)?;

    // add_task(&mut tasks, "Task".to_string());
    // add_task(&mut tasks, "Task".to_string());
    // add_task(&mut tasks, "Task".to_string());
    toggle_task(&mut tasks, 0);

    display_tasks(&tasks);

    write_tasks(&mut file, &mut tasks)?;

    Ok(())
}
