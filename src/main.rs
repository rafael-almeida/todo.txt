use clap::{Parser, Subcommand};
use std::fs::{File, OpenOptions};
use std::io::{self, prelude::*};
use std::num::ParseIntError;
use std::str::{FromStr, ParseBoolError};

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
        title,
        completed: false,
    };

    tasks.push(task);
}

fn remove_task(tasks: &mut Vec<Task>, id: isize) {
    tasks.retain(|task| task.id != id);
}

fn display_tasks(tasks: &[Task]) {
    let head: [&str; 2] = ["id", "title"];
    let mut rows_len: [usize; 2] = head.map(|x| x.len());

    for task in tasks {
        rows_len[0] = rows_len[0].max(task.id.to_string().len());
        rows_len[1] = rows_len[1].max(task.title.to_string().len());
    }

    let a = rows_len[0];
    let b = rows_len[1];
    let w = a + b + 7;

    println!("{:->w$}", "-");
    println!("| {:a$} | {:b$} |", head[0], head[1]);
    println!("| {:->a$} | {:->b$} |", "-", "-");

    for t in tasks {
        println!("| {:a$} | {:b$} |", t.id, t.title,);
    }

    println!("{:->w$}", "-");
}

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(alias = "ls")]
    List {},

    #[clap(alias = "a")]
    Add { name: Option<String> },

    #[clap(alias = "rm")]
    Remove { id: Option<String> },
}

fn main() -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .read(true)
        .append(true)
        .open("todo.txt")?;

    let mut tasks = read_tasks(&mut file)?;
    let cli = Cli::parse();

    match &cli.command {
        Commands::List {} => {
            display_tasks(&tasks);
        }
        Commands::Add { name } => {
            if let Some(name_str) = name {
                add_task(&mut tasks, name_str.to_string());
                display_tasks(&tasks);
                write_tasks(&mut file, &mut tasks)?;
            } else {
                println!("Title is required");
            }
        }
        Commands::Remove { id } => {
            if let Some(id_str) = id {
                match id_str.parse::<isize>() {
                    Ok(x) => {
                        remove_task(&mut tasks, x);
                        display_tasks(&tasks);
                        write_tasks(&mut file, &mut tasks)?;
                    }

                    Err(x) => println!("Unable to parse id - {}", x),
                }
            } else {
                println!("Id is required");
            }
        }
    }

    Ok(())
}
