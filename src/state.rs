use axum::extract::FromRef;
use leptos::leptos_config::LeptosOptions;

#[derive(Clone)]
pub struct BioState {
    pub bio_base_url: String,
    pub http: reqwest::Client,
}

#[derive(Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub bio: BioState,
}

// leptos_routes 用：LeptosOptions を AppState から取り出す
impl FromRef<AppState> for LeptosOptions {
    fn from_ref(s: &AppState) -> LeptosOptions {
        s.leptos_options.clone()
    }
}

// axum State 抽出用：BioState を AppState から取り出す
impl FromRef<AppState> for BioState {
    fn from_ref(s: &AppState) -> BioState {
        s.bio.clone()
    }
}
