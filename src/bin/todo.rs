use std::io::{self, Write};
use colored::*;

struct Task {
    name: String,
    completed: bool,
}

fn main() {
    println!("");
    println!("{}", "           Welcome to the To-Do List           ".red().on_blue().bold().italic());

    let mut tasks: Vec<Task> = Vec::new();

    loop {
       print!("{}", "\nEnter a new task (or type 'quit' to exit or 'list' to show current tasks): ".cyan());
       io::stdout().flush().unwrap();

       let mut input = String::new();
       io::stdin()
           .read_line(&mut input)
           .expect("Failed to read line");

       let input = input.trim();

       if input.to_lowercase() == "quit" {
           break;
       }

       if input.to_lowercase() == "list" {
           println!("{}", "          Your current tasks          ".green().on_yellow());

           if tasks.is_empty() {
               println!("{}", "You do not have any task!! GO RELAX".magenta().italic());
           }
           else {
               for (index, task) in tasks.iter().enumerate() {
                   let status_box = if task.completed { "[X]" } else { "[ ]" };

                   println!("{} {} {}", index + 1, ".", status_box, task.name);
               }
           }

           continue;
       }

       let new_task = Task { name : input.to_string(), completed : false };

       tasks.push(new_task);

       println!("{}", "The task was has been added.".magenta().italic());

       let out_message = format!("You now have {} tasks in your list.", tasks.len());

       println!("{}", out_message.yellow());

    }

    println!("{}", "Exiting... Goodbye!!!!".red().italic());
    println!("");
}
