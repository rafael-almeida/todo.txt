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

fn add_task(tasks: &mut Vec<Task>, title: String) -> std::io::Result<()> {
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

    Ok(())
}

fn toggle_task(tasks: &mut Vec<Task>, id: isize) -> std::io::Result<()> {
    for task in tasks {
        if task.id == id {
            task.completed = !task.completed;
        }
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .read(true)
        .append(true)
        .open("data.txt")?;

    let mut tasks = read_tasks(&mut file)?;

    add_task(&mut tasks, "Task".to_string())?;
    add_task(&mut tasks, "Task".to_string())?;
    add_task(&mut tasks, "Task".to_string())?;
    toggle_task(&mut tasks, 0)?;

    write_tasks(&mut file, &mut tasks)?;

    println!("{:#?}", tasks);

    Ok(())
}
