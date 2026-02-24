use std::io::{self, Write};
use std::fs;
use colored::*;

struct Task {
    name: String,
    completed: bool,
}

fn save_tasks(tasks: &Vec<Task>) {

    let mut file_content = String::new();

    for task in tasks {
        let line = format!("{} | {}\n", task.name, task.completed);
        file_content.push_str(&line);
    }
    
    fs::write("todo.txt", file_content).expect("Failed to save tasks to file.")

}

fn load_tasks() -> Vec<Task> {

    let mut tasks = Vec::new();

    if let Ok(content) = fs::read_to_string("todo.txt") {

        for line in content.lines() {

            let parts: Vec<&str> = line.split('|').collect();

            if parts.len() == 2 {

                let name = parts[0].trim().to_string();
                let completed = parts[1].trim() == "true";

                tasks.push(Task {name, completed})

            }
        }
    }
    
    tasks
}

fn main() {
    println!("");
    println!("{}", "           Welcome to the To-Do List           ".red().on_blue().bold().italic());

    let mut tasks: Vec<Task> = load_tasks();

    loop {
       print!("{}", "\nEnter a command or task (type 'help' for options): ".cyan());
       io::stdout().flush().unwrap();

       let mut input = String::new();
       io::stdin()
           .read_line(&mut input)
           .expect("Failed to read line");

       let input = input.trim();

       if input.to_lowercase() == "quit" {
           break;
       }

       if input.to_lowercase() == "help" {
           println!("\n{}", "         --- Available Commands ---".yellow().bold());
           println!("  {} - View your current tasks", "list".cyan());
           println!("  {} - Mark a task as complete (e.g., 'done 1')", "done <#>\t".cyan());
           println!("  {} - Exit the program", "quit".cyan());
           println!("  (Anything else you type will be added as a new task)");
           continue;
       }

       if input.to_lowercase() == "list" {
           println!("\n{}", "          Your current tasks          ".green().bold().italic());
           println!("");

           if tasks.is_empty() {
               println!("{}", "You do not have any task!! GO RELAX".magenta().italic());
           } else {
               for (index, task) in tasks.iter().enumerate() {
                   let status_box = if task.completed { "[✅]" } else { "[❌]" };
                   println!("{} {} {}", index + 1, status_box, task.name);
               }
           }
           continue;
       }

       if input.to_lowercase().starts_with("done ") {
           let num_str = input[5..].trim();

           match num_str.parse::<usize>() {
               Ok(task_num) => {
                   if task_num > 0 && task_num <= tasks.len() {

                       let index = task_num - 1;

                       tasks[index].completed = true;
                       save_tasks(&tasks);

                       println!("{}", "Task marked as completed.".green().bold());
                   } else {
                       println!("{}", "Oops!! That task number doesn't exist".red());
                   }
               }
               Err(_) => {
                   println!("{}", "Please provide a valid task number, like 'done 1'".red());
               }
           }
           continue;
       }

       let new_task = Task { name: input.to_string(), completed: false };
       tasks.push(new_task);
       save_tasks(&tasks);

       println!("{}", "The task has been added.".magenta().italic());
       let out_message = format!("You now have {} tasks in your list.", tasks.len());
       println!("{}", out_message.yellow());
    }

    println!("{}", "\nExiting... Goodbye!!!!".red().italic());
    println!("");
}
