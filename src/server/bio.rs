#![cfg(feature = "ssr")]

use axum::{Router, routing::post};
use crate::routes;

pub fn router() -> Router {
    Router::new()
        .route("/api/bio/foxp2/analyze", post(routes::bio::foxp2_analyze))
}
