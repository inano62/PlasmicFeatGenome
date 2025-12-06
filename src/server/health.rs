// src/server/health.rs（場所は適当に）
use leptos::*;
use server_fn::ServerFnError;

#[server(GetHealth, "/api")]
pub async fn get_health() -> Result<String, ServerFnError> {
    // ここで Postgres / Bio / Llama に ping
    // 成功したら JSON 文字列を返すイメージ
    Ok(r#"{"db":"ok","bio":"ok","llama":"ok"}"#.to_string())
}
