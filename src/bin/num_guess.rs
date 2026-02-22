use std::cmp::Ordering;
use rand::Rng;
use std::io::{self, Write};

fn main(){

    println!("Welcome to the number guessing game.");
    println!("------------------------------------");

    let sec_num: i32 = rand::thread_rng().gen_range(1..=100);

    loop {
        print!("Please enter your guess: ");
        io::stdout().flush().unwrap();

        let mut guess_input = String::new();
        io::stdin()
            .read_line(&mut guess_input)
            .expect("error: enter a valid number.");

        let guess: i32 = match guess_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That's not a number! Please try again.");
                continue;
            }
        };

        match guess.cmp(&sec_num){
            Ordering::Less => println!("Too small."),
            Ordering::Greater => println!("Too big."),
            Ordering::Equal => {
                println!("lessssgooooooooooo, you nailed it, the number was indeed {}.", sec_num);
                break;
            }
        }
    }
}
