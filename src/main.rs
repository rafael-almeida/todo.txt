use std::fmt;

struct Task {
    id: String,
    title: String,
    completed: bool,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Task {{id: {}, title: \"{}\", completed: {}}}",
            self.id, self.title, self.completed
        )
    }
}

fn main() {
    let task = Task {
        id: "1".to_string(),
        title: "First Task".to_string(),
        completed: false,
    };

    println!("Hello, world!");
    println!("{}", task);
}
