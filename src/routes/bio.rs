use axum::{
    Extension,
    Json,
    http::StatusCode,
    response::IntoResponse,
};

use crate::state::BioState;
use crate::shared::bio_types::{AnalyzeReq, AnalyzeRes};

pub async fn foxp2_analyze(
    Extension(st): Extension<BioState>,
    Json(req): Json<AnalyzeReq>,
) -> impl IntoResponse {
    let url = format!(
        "{}/foxp2/analyze",
        st.bio_base_url.trim_end_matches('/')
    );

    let r = match st.http.post(url).json(&req).send().await {
        Ok(r) => r,
        Err(e) => return (StatusCode::BAD_GATEWAY, e.to_string()).into_response(),
    };

    if !r.status().is_success() {
        let body = r.text().await.unwrap_or_default();
        return (StatusCode::BAD_GATEWAY, body).into_response();
    }

    match r.json::<AnalyzeRes>().await {
        Ok(json) => (StatusCode::OK, Json(json)).into_response(),
        Err(e) => (StatusCode::BAD_GATEWAY, e.to_string()).into_response(),
    }
}
