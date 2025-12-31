#![cfg(feature = "ssr")]

use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};

pub async fn router() -> Router {
    Router::new().route("/analyze", post(analyze))
}

#[derive(Deserialize)]
struct AnalyzeReq {
    text: String,
}

#[derive(Serialize)]
struct AnalyzeRes {
    ok: bool,
    summary: String,
}

async fn analyze(Json(req): Json<AnalyzeReq>) -> Json<AnalyzeRes> {
    // TODO: ここで llama / biopython 相当を呼ぶ
    // 例: reqwestでllama-service叩く / pythonサービス叩く / DB読む
    Json(AnalyzeRes {
        ok: true,
        summary: format!("received {} chars", req.text.len()),
    })
}
