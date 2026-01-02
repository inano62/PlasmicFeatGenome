use axum::{Extension, Router};
use leptos::leptos_config::get_configuration;
use leptos_axum::{generate_route_list, LeptosRoutes};

use plasmic_feat_genome::app::App;
use plasmic_feat_genome::server::bio as bio_server;
use plasmic_feat_genome::state::BioState;

#[tokio::main]
async fn main() {
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let leptos_route_list = generate_route_list(App);

    let bio_base_url =
        std::env::var("BIO_URL").unwrap_or_else(|_| "http://bio-service:8000".into());
    let bio_state = BioState {
        bio_base_url,
        http: reqwest::Client::new(),
    };

    // bio_router は state不要なので、with_state(()) で “外側のstate型” だけ揃える
    let bio_router = bio_server::router()
        .layer(Extension(bio_state))
        .with_state(()); // ← 重要

    // Router<LeptosOptions> を明示して組み立てる
    let app = Router::<leptos::leptos_config::LeptosOptions>::new()
        .merge(bio_router)
        .leptos_routes(&leptos_options, leptos_route_list, App)
        .with_state(leptos_options.clone()); // ← serve できる Router<()> に落とす

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
#[allow(dead_code)]
#[derive(Clone, serde::Deserialize)]
struct AnalyzeRes {
    record_id: String,
    length_nt: i32,
    gc_percent: f64,
    facts: String,
}
