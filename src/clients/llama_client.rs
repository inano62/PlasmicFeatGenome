// src/clients/llama_client.rs
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct LlamaRequest {
    prompt: String,
}

#[derive(Deserialize)]
struct LlamaResponse {
    output: String,
}

pub async fn call_llama(prompt: String) -> anyhow::Result<String> {
    let client = Client::new();

    // ★ まずは service 名 / port=8000 に合わせる
    let base = std::env::var("LLAMA_URL")
        .unwrap_or_else(|_| "http://llama-service:8000".to_string());

    let resp = client
        .post(format!("{base}/generate"))
        .json(&LlamaRequest { prompt })
        .send()
        .await?
        .error_for_status()? // 4xx/5xx を弾く
        .json::<LlamaResponse>()
        .await?;

    Ok(resp.output)
}
