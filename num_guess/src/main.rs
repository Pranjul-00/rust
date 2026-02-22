use std::cmp::Ordering;
use std::io::{self, Write};

fn main(){

    println!("Welcome to the number guessing game.");
    println!("------------------------------------");

    let sec_num: i32 = 42;

    loop {
        print!("Please enter your guess: ");
        io::stdout().flush().unwrap();

        let mut guess_input = String::new();
        io::stdin()
            .read_line(&mut guess_input)
            .expect("error: enter a valid number.");

        let guess: i32 = guess_input.trim().parse().expect("error: please provide valid input.");

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

