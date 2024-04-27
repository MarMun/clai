use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct AiResponse {
    context: Vec<u64>,
    created_at: String,
    done: bool,
    load_duration: u64,
    eval_count: u64,
    eval_duration: u64,
    model: String,
    prompt_eval_count: u64,
    prompt_eval_duration: u64,
    response: String,
    total_duration: u64,
}

pub async fn call(prompt: &String, model: Option<&str>) -> Result<String, Box<dyn Error>> {
    // Set default / fallback model
    let model = model.unwrap_or("codellama:13b");

    // Prepare request body
    let body = json!({
      "model": model,
      "stream": false,
      "prompt": prompt
    });

    // Send post request
    let url = "http://localhost:11434/api/generate"; // replace with your local server URL

    let resp = Client::new()
        .post(url)
        .json(&body)
        .header("Content-Type", "application/json")
        .send()
        .await?;

    // let result: HashMap<String, serde_json::Value> = resp.json().await?;
    let result: AiResponse = resp.json().await?;

    // println!("{:?}", result);
    // println!("{0:#?}", result.response);

    Ok(result.response)
}
