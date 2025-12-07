use leptos::*;
use server_fn::ServerFnError;
use serde_json::json;

#[server(LlamaAsk, "/api")]
pub async fn llama_ask(prompt: String) -> Result<String, ServerFnError> {
    let client = reqwest::Client::new();

    let resp = client
        .post("http://plasmic-llama:8000/ask")
        .json(&json!({ "prompt": prompt }))
        .send()
        .await;

    let resp = match resp {
        Ok(r) => r,
        Err(e) => {
            return Err(ServerFnError::ServerError(format!(
                "Request error: {}",
                e
            )))
        }
    };

    println!("[LlamaAsk] prompt = {}", prompt);

    let text = resp.text().await.unwrap_or_default();
    Ok(text)
}
