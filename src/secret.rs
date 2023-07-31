use super::gpt_completion;

use rand::Rng;

pub fn generate_secret_number(start: u32, end: u32) -> u32 {
    rand::thread_rng().gen_range(start..=end)
}

pub fn generate_secret_number_gpt(start: u32, end: u32) -> u32 {
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

pub fn handler_secret_number(gpt: bool) -> fn(u32, u32) -> u32 {
    if gpt {
        println!("OPENAI TOKEN found!");
        println!("Generating secret number with GPT3.5");
        return generate_secret_number_gpt;
    }
    println!("OPENAI TOKEN not found!");
    println!("Generating secret number with local CPU");
    generate_secret_number
}

pub fn gpt_clue(secret_number: u32) -> String {
    match gpt_completion(format!("Tell me a funny clue that helps me to guess the secret number. Just returns the clue without the secret number nor the clue's explanation.: {}", secret_number)){
        Ok(clue) => {
            return format!("GPT Clue: {}", clue);
        },
        Err(_) => {
            return format!("Sorry, at this moment, I can't give you a clue.");
        },
    };
}
