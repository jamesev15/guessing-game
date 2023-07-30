mod gpt;
mod secret;

use gpt::gpt_completion;
use secret::handler_secret_number;

use std::cmp::Ordering;
use std::env;
use std::io;

struct Game {
    secret_number: u32,
}

impl Game {
    fn new() -> Self {
        println!("Welcome to the guessing game powered by GPT");

        let mut gpt: bool = false;
        let openai_token = env::var("OPENAI_TOKEN").unwrap_or_default();

        if openai_token != "".to_string() {
            gpt = true;
        }

        let fn_handler = handler_secret_number(gpt);
        let secret_number = fn_handler(0, 50);

        if gpt {
            match gpt_completion(format!("Tell me a funny clue that helps me to guess the secret number. Just returns the clue without the secret number nor the clue's explanation.: {}", secret_number)){
                Ok(clue) => {
                    println!("GPT Clue: {}", clue);
                },
                Err(e) => {
                    println!("Error generating clue: {}", e);
                },
            };
        }

        Self { secret_number }
    }

    fn start(&self) {
        loop {
            println!("-- Input your guess -- :");

            let mut guess = String::new();

            io::stdin().read_line(&mut guess).expect("Error");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("It needs to be a number!");
                    continue;
                }
            };

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
