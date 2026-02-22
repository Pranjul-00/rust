use std::cmp::Ordering;
use rand::Rng;
use std::io::{self, Write};
use colored::*;

fn main(){

    println!("{}", "Welcome to the number guessing game.".cyan().bold());
    println!("{}", "------------------------------------".cyan());

    let sec_num: i32 = rand::thread_rng().gen_range(1..=1000);

    loop {
        print!("Please enter your guess: ");
        io::stdout().flush().unwrap();

        let mut guess_input = String::new();
        io::stdin()
            .read_line(&mut guess_input)
            .expect("Failed to read line.");

        let guess: i32 = match guess_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "That's not a number! Please try again.".red().bold());
                continue;
            }
        };

        match guess.cmp(&sec_num){
            Ordering::Less => println!("{}", "Too small.".yellow()),
            Ordering::Greater => println!("{}", "Too big.".magenta()),
            Ordering::Equal => {
                let victory_message = format!("lesssgoooooooooo, you nailed it, the number was indeed {}.", sec_num);
                println!("{}", victory_message.green().bold());
                break;
            }
        }
    }
}
