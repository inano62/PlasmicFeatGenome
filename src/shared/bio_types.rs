use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyzeReq {
    pub fasta: String,
    pub prompt: Option<String>,
    pub max_tokens: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyzeRes {
    pub record_id: String,
    pub length_nt: i32,
    pub gc_percent: f64,
    pub facts: String,
    pub summary: Option<String>,
}
