// use rand::Rng;
use serde::Deserialize;
use serde::Serialize;
use std::cmp::Ordering;
use std::io;
use std::env;
use serde_json;
use reqwest::blocking;

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct GptMessage{
    role: String,
    content: String,
}
#[derive(Serialize)]
struct GptPayload{
    model: String,
    temperature: f32,
    messages: Vec<GptMessage>
}

#[derive(Debug)]
#[derive(Deserialize)]
struct GptChoice{
    message: GptMessage,
}

#[derive(Debug)]
#[derive(Deserialize)]
struct GptResponse{
    choices: Vec<GptChoice>
}

// fn generate_secret_number(start: u32, end: u32) -> u32{
//     rand::thread_rng().gen_range(start..=end)
// }

fn generate_secret_number_gpt(start: u32, end: u32) -> u32{

    let gpt_payload = GptPayload{
        model: "gpt-3.5-turbo".to_string(),
        temperature: 0.7,
        messages: vec![GptMessage{
            role: "user".to_string(),
            content: format!("Return just a random number between {} and {} without explanations.", start, end),
        }]
    };

    let gpt_payload = serde_json::to_string(&gpt_payload).expect("Error converting to JSON");

    // Reading OPENAI_TOKEN env var
    let openai_token = env::var("OPENAI_TOKEN").unwrap_or_default();

    let client = reqwest::blocking::Client::new();

    let response = client
            .post("https://api.openai.com/v1/chat/completions".to_string())
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", openai_token))
            .body(gpt_payload)
            .send()
            .expect("Error calling GPT");

    if response.status().is_success(){
        let response_text = response.text().unwrap_or_default();
        let gpt_response: GptResponse = serde_json::from_str(&response_text).expect("Error parsing GPT response");
        let secret_number: u32 = gpt_response.choices[0].message.content.parse().unwrap_or(50);

        return secret_number;
    }

    50


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
        let secret_number = generate_secret_number_gpt(1, 100);

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