use clap::{Arg, Command};
use reqwest::Client;
use serde_json::json;

#[tokio::main]
async fn main() {
    let matches = Command::new("bonzai")
        .about("Query a local Ollama model")
        .arg(
            Arg::new("prompt")
                .help("The prompt to send to the model")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("model")
                .short('m')
                .long("model")
                .help("Specify the Ollama model (default: 'qwen2.5')")
                .num_args(1),
        ) // Replaces `.takes_value(true)`
        .get_matches();

    let prompt = matches.get_one::<String>("prompt").unwrap();
    let model = matches
        .get_one::<String>("model")
        .map_or("qwen2.5", String::as_str); // Replaces `.value_of()`

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
