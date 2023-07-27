use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn generate_secret_number(start: u32, end: u32) -> u32{
    rand::thread_rng().gen_range(start..=end)
}

struct Game {
    secret_number: u32,
}

impl Game{
    fn new() -> Self{
        // println! is a macro
        println!("Initializing game");
        Self { secret_number: generate_secret_number(1, 100) }
    }

    fn start(&self) {
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
            match guess.cmp(&self.secret_number) {
                Ordering::Less => println!("To small"),
                Ordering::Greater => println!("To big"),
                Ordering::Equal => {
                    println!("You win!");
                    break;
                }
            }
        }
    }
}

fn main() {
    
    let game = Game::new();
    game.start();

}