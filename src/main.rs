use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::env;


fn generate_secret_number(start: u32, end: u32) -> u32{
    rand::thread_rng().gen_range(start..=end)
}

fn read_debug_env_var(env_var: &str) -> bool{
    env::var(env_var)
            .unwrap_or("false".to_string())
            .parse::<bool>()
            .unwrap_or_default()
}

struct Game {
    debug: bool,
    secret_number: u32,
}

impl Game{

    const GAME_DEBUG_NAME: &str = "GAME_DEBUG";

    fn new() -> Self{
        // println! is a macro
        println!("Initializing game");

        let secret_number = generate_secret_number(1, 100);

        Self { 
            debug: read_debug_env_var(Game::GAME_DEBUG_NAME),
            secret_number,
        }
    }

    fn start(&self) {

        if self.debug {
            println!("[DEBUG] Secret number: {}", self.secret_number);
        }

        loop {
            println!("-- Input your guess -- :");
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