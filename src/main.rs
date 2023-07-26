use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // println! is a macro
    println!("* Guess the number *");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("-- Input your guess -- ");

        // variable to mutate
        let mut guess = String::new();
        
        // read_line() returns a Result enum: Ok, Err
        io::stdin().read_line(&mut guess).expect("Error");

        // overshadow guess String variable
        // parse() returns a Result enum: Ok, Err
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("It needs to be a number!");
                continue;
            }
        };

        // Compare two numbers using pattern matching
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("To small"),
            Ordering::Greater => println!("To big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}