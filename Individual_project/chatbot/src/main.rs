use reqwest;
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::io;

#[derive(Serialize)]
struct ChatInput {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
}

#[derive(Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ChatOutput {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: Message,
}

async fn send_message(client: &reqwest::Client, message: &str) -> Result<String, Box<dyn Error>> {
    let url = "https://api.openai.com/v1/chat/completions";

    let chat_input = ChatInput {
        model: "gpt-3.5-turbo".to_string(),
        messages: vec![Message {
            role: "user".to_string(),
            content: message.to_string(),
        }],
        temperature: 0.7,
    };

    let response = client
        .post(url)
        .json(&chat_input)
        .send()
        .await?
        .error_for_status()?;

    let chat_output: ChatOutput = response.json().await?;
    let generated_text = chat_output.choices[0].message.content.clone();
    Ok(generated_text)
}

fn api_key_headers(api_key: &str) -> Result<reqwest::header::HeaderMap, Box<dyn std::error::Error>> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("Authorization", format!("Bearer {}", api_key).parse().unwrap());
    Ok(headers)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("OPENAI_API_KEY")?;
    let client = reqwest::Client::builder()
        .default_headers(api_key_headers(&api_key)?)
        .build()?;

    println!("Chatbot is ready. Type a message:");

    loop {
        let mut message = String::new();
        io::stdin().read_line(&mut message)?;
        let message = message.trim();

        if message == "quit" || message == "exit" {
            break;
        }

        match send_message(&client, message).await {
            Ok(response) => println!("Bot: {}", response),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    Ok(())
}
