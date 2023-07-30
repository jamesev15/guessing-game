use reqwest;
use serde::Deserialize;
use serde::Serialize;
use serde_json;
use std::env;

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

pub fn gpt_completion(prompt: String) -> Result<String, String> {
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
    }

    let response_text = match response.text() {
        Ok(r) => r,
        Err(_) => return Err("Error reading GPT response".to_string()),
    };

    let gpt_response: GptResponse = match serde_json::from_str(&response_text) {
        Ok(g) => g,
        Err(_) => return Err("Error converting to json".to_string()),
    };

    Ok(gpt_response.choices[0].message.content.clone())
}
