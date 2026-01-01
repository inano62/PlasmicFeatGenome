use axum::{extract::State, http::StatusCode, Json};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct AppState {
    pub bio_base_url: String,
    pub http: Client,
}

#[derive(Deserialize)]
pub struct AnalyzeReq {
    pub fasta: String,
    pub prompt: Option<String>,
    pub max_tokens: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct AnalyzeRes {
    pub record_id: String,
    pub length_nt: i32,
    pub gc_percent: f64,
    pub facts: String,
    pub summary: Option<String>,
}

pub async fn foxp2_analyze(
    State(st): State<AppState>,
    Json(req): Json<AnalyzeReq>,
) -> Result<Json<AnalyzeRes>, (StatusCode, String)> {
    let url = format!("{}/foxp2/analyze", st.bio_base_url.trim_end_matches('/'));

    let resp = st.http
        .post(url)
        .json(&req)
        .send()
        .await
        .map_err(|e| (StatusCode::BAD_GATEWAY, format!("bio request error: {e}")))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err((StatusCode::BAD_GATEWAY, format!("bio http error: {status} {body}")));
    }

    let json = resp
        .json::<AnalyzeRes>()
        .await
        .map_err(|e| (StatusCode::BAD_GATEWAY, format!("bio json error: {e}")))?;

    Ok(Json(json))
}
