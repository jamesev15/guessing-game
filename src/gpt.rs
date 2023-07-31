use reqwest;
use serde::Deserialize;
use serde::Serialize;
use serde_json;
use std::env;

#[derive(Serialize, Deserialize)]
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

#[derive(Deserialize)]
struct GptChoice {
    message: GptMessage,
}

#[derive(Deserialize)]
struct GptResponse {
    choices: Vec<GptChoice>,
}

pub fn gpt_completion(prompt: String) -> Result<String, String> {
    // Checking OPENAI TOKEN
    let openai_token = match env::var("OPENAI_TOKEN") {
        Ok(var) => var,
        Err(_) => return Err("OPENAI TOKEN not found.".to_string()),
    };

    // Building gpt payload based on the user prompt
    let gpt_payload = GptPayload {
        model: "gpt-3.5-turbo".to_string(),
        temperature: 0.7,
        messages: vec![GptMessage {
            role: "user".to_string(),
            content: prompt,
        }],
    };

    // Converting to json
    let gpt_payload = match serde_json::to_string(&gpt_payload) {
        Ok(s) => s,
        Err(_) => return Err("Error converting gpt payload to JSON".to_string()),
    };

    // Making request to GPT
    let client = reqwest::blocking::Client::new();

    let response = match client
        .post("https://api.openai.com/v1/chat/completions".to_string())
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", openai_token))
        .body(gpt_payload)
        .send()
    {
        Ok(r) => r,
        Err(_) => return Err("Error requesting to GPT".to_string()),
    };

    if !response.status().is_success() {
        return Err(format!(
            "Error requesting to GPT. Status code: {}",
            response.status()
        ));
    }

    let response_text = match response.text() {
        Ok(r) => r,
        Err(_) => return Err("Error reading GPT response".to_string()),
    };

    // Converting gpt's response to GptReponse struct
    let gpt_response: GptResponse = match serde_json::from_str(&response_text) {
        Ok(g) => g,
        Err(_) => return Err("Error converting gpt'response to json".to_string()),
    };

    // Returning validated gpt's response
    Ok(gpt_response.choices[0].message.content.clone())
}
