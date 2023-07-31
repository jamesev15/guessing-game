mod gpt;
mod secret;

use gpt::gpt_completion;
use secret::{gpt_clue, handler_secret_number};

use std::cmp::Ordering;
use std::env;
use std::io;

struct Game {
    secret_number: u32,
}

impl Game {
    fn new(start: u32, end: u32) -> Self {
        println!("Welcome to the guessing game powered by GPT");
        println!("Guess a secret number between {} - {}", start, end);

        let openai_token = env::var("OPENAI_TOKEN").unwrap_or_default();

        let gpt = if &openai_token != "" { true } else { false };

        let fn_handler = handler_secret_number(gpt);
        let secret_number = fn_handler(start, end);

        if gpt {
            println!("{}", gpt_clue(secret_number));
        }

        Self { secret_number }
    }

    fn start(&self) {
        loop {
            println!("Input your guess: ");

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
    let game = Game::new(0, 50);
    game.start();
}
