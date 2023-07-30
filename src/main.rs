use rand::Rng;
use reqwest;
use serde::Deserialize;
use serde::Serialize;
use serde_json;
use std::cmp::Ordering;
use std::env;
use std::io;

#[derive(Debug, Serialize, Deserialize)]
struct GptMessage {
    role: String,
    content: String,
}
#[derive(Serialize)]
struct GptPayload {
    model: String,
    temperature: f32,
    messages: Vec<GptMessage>,
}

#[derive(Debug, Deserialize)]
struct GptChoice {
    message: GptMessage,
}

#[derive(Debug, Deserialize)]
struct GptResponse {
    choices: Vec<GptChoice>,
}

fn gpt_completion(prompt: String) -> Result<String, String> {
    let openai_token = match env::var("OPENAI_TOKEN") {
        Ok(var) => var,
        Err(_) => return Err("OPENAI Token not found.".to_string()),
    };

    let gpt_payload = GptPayload {
        model: "gpt-3.5-turbo".to_string(),
        temperature: 0.7,
        messages: vec![GptMessage {
            role: "user".to_string(),
            content: prompt,
        }],
    };

    let gpt_payload = match serde_json::to_string(&gpt_payload) {
        Ok(s) => s,
        Err(_) => return Err("error generating gpt payload".to_string()),
    };

    let client = reqwest::blocking::Client::new();

    let response = match client
        .post("https://api.openai.com/v1/chat/completions".to_string())
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", openai_token))
        .body(gpt_payload)
        .send()
    {
        Ok(r) => r,
        Err(_) => return Err("Error calling OPENAI URL".to_string()),
    };

    if !response.status().is_success() {
        return Err(format!("Error status code: {}", response.status()));
    } else {
        let response_text = match response.text() {
            Ok(r) => r,
            Err(_) => return Err("Error reading GPT response".to_string()),
        };

        let gpt_response: GptResponse = match serde_json::from_str(&response_text) {
            Ok(g) => g,
            Err(_) => return Err("Error converting to json".to_string()),
        };

        return Ok(gpt_response.choices[0].message.content.clone());
    };
}

fn generate_secret_number(start: u32, end: u32) -> u32 {
    rand::thread_rng().gen_range(start..=end)
}

fn generate_secret_number_gpt(start: u32, end: u32) -> u32 {
    let secret_number_default: u32 = 50;

    let gpt_response = match gpt_completion(format!(
        "Return just a random number between {} and {} without explanations.",
        start, end
    )) {
        Ok(r) => r,
        Err(e) => {
            println!("Error calling gpt {}", e);
            return secret_number_default;
        }
    };

    let secret_number: u32 = match gpt_response.parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Error parsing number from gpt");
            return secret_number_default;
        }
    };

    secret_number
}

fn handler_secret_number() -> fn(u32, u32) -> u32 {
    let openai_token = env::var("OPENAI_TOKEN").unwrap_or_default();

    if openai_token == "".to_string() {
        println!("OPENAI TOKEN not found!");
        println!("Generating secret number with local CPU");
        generate_secret_number
    } else {
        println!("OPENAI TOKEN found!");
        println!("Generating secret number with GPT3.5");
        generate_secret_number_gpt
    }
}

struct Game {
    secret_number: u32,
}

impl Game {
    fn new() -> Self {
        println!("Welcome to the guessing game powered by GPT");

        let fn_handler = handler_secret_number();
        let secret_number = fn_handler(0, 50);

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
