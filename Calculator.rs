use std::io::{self, Write};

fn main(){

    println!("WELCOME TO THIS BASIC CALCULATOR");
    println!("--------------------------------");

    print!("Enter the 1st number: ");
    io::stdout().flush().unwrap();
    
    let mut input1 = String::new();
    io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read line!!!");
    
    let n1: i32 = input1.trim().parse().expect("Please enter a valid number!!!"); 

    print!("Enter the 2nd number: ");
    io::stdout().flush().unwrap();
    
    let mut input2 = String::new();
    io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read line!!!");
    
    let n2: i32 = input2.trim().parse().expect("Please enter a valid number!!!");

    println!("---Main Menu---");
    println!("1. Sum");
    println!("2. Difference");
    println!("3. Divison");
    println!("4. Product");
    println!("5. Exit");
    println!("---------------");
    print!("Enter your choice (1,2,3,4 or 5): ");
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line!!!");

    match choice.trim(){
        "1" => println!("Sum of {} and {} is {}.", n1, n2, sum(n1, n2)),
        "2" => println!("Difference between {} and {} is {}.", n1, n2, difference(n1, n2)),
        "3" => println!("{}/{} is {}.", n1, n2, division(n1, n2)),
        "4" => println!("Product of {} and {} is {}.", n1, n2, product(n1, n2)),
        "5" => println!("Exiting.... Goodbye!"),
        _ => println!("Invalid choice. Please select 1,2,3,4 or 5."),
    }

}

fn sum(n1: i32, n2: i32) -> i32 {
    n1 + n2
}

fn difference(n1: i32, n2: i32) -> i32 {
    (n1-n2).abs()
}

fn division(n1: i32, n2: i32) -> i32 {
    n1/n2
}

fn product(n1: i32, n2: i32) -> i32 {
    n1*n2
}
