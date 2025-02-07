use clap::{Arg, Command};
use reqwest::Client;
use serde_json::json;
use std::io::{self, Write};
use tokio;

#[tokio::main]
async fn main() {
    let matches = Command::new("bonzai")
        .about("Query a local Ollama model")
        .arg(
            Arg::new("prompt")
                .help("The prompt to send to the model")
                .required(false)
                .index(1),
        )
        .arg(
            Arg::new("model")
                .short('m')
                .long("model")
                .help("Specify the Ollama model (default: 'qwen2.5:14b')")
                .num_args(1),
        )
        .arg(
            Arg::new("chat")
                .short('c')
                .long("chat")
                .help("Start interactive chat mode")
                .action(clap::ArgAction::SetTrue),
        ) // Ensure proper flag behavior
        .get_matches();

    let model = matches
        .get_one::<String>("model")
        .map_or("qwen2.5:14b", String::as_str);

    if matches.get_flag("chat") {
        start_chat(model).await;
    } else if let Some(prompt) = matches.get_one::<String>("prompt") {
        send_query(model, prompt).await;
    } else {
        eprintln!("Error: No input provided. Use --chat for interactive mode or provide a query.");
    }
}

async fn start_chat(model: &str) {
    let client = Client::new();
    let url = "http://localhost:11434/api/generate";

    println!("🐵 Bonzai Chat Mode Activated! Type 'exit' to quit.");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let prompt = input.trim();

        if prompt.eq_ignore_ascii_case("exit") {
            println!("👋 Goodbye!");
            break;
        }

        let body = json!({
            "model": model,
            "prompt": prompt,
            "stream": false
        });

        match client.post(url).json(&body).send().await {
            Ok(response) => {
                if let Ok(text) = response.json::<serde_json::Value>().await {
                    if let Some(output) = text["response"].as_str() {
                        println!("Bonzai: {}", output);
                    }
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}

async fn send_query(model: &str, prompt: &str) {
    let client = Client::new();
    let url = "http://localhost:11434/api/generate";

    let body = json!({
        "model": model,
        "prompt": prompt,
        "stream": false
    });

    match client.post(url).json(&body).send().await {
        Ok(response) => {
            if let Ok(text) = response.json::<serde_json::Value>().await {
                if let Some(output) = text["response"].as_str() {
                    println!("{}", output);
                }
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
