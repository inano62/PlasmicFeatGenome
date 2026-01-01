use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct AppState {
    pub bio_base_url: String,
    pub http: Client,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyzeReq {
    pub fasta: String,
    pub prompt: Option<String>,
    pub max_tokens: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyzeReq {
    pub fasta: String,
    pub prompt: Option<String>,
    pub max_tokens: Option<i32>,
}
pub async fn foxp2_analyze(
    State(st): State<AppState>,
    Json(req): Json<AnalyzeReq>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let url = format!("{}/foxp2/analyze", st.bio_base_url.trim_end_matches('/'));

    let r = st.http
        .post(url)
        .json(&req)
        .send()
        .await
        .map_err(|e| (StatusCode::BAD_GATEWAY, format!("bio request error: {e}")))?;

    if !r.status().is_success() {
        let code = r.status();
        let body = r.text().await.unwrap_or_default();
        return Err((StatusCode::BAD_GATEWAY, format!("bio http {code}: {body}")));
    }

    let json = r
        .json::<AnalyzeRes>()
        .await
        .map_err(|e| (StatusCode::BAD_GATEWAY, format!("bio json error: {e}")))?;

    Ok(Json(json))
}
