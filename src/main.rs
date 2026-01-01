// src/main.rs
use axum::{routing::post, Router};
use leptos::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use reqwest::Client;

mod app;
mod routes;

use crate::app::App;

#[derive(Clone)]
pub struct AppState {
    pub leptos_options: leptos::LeptosOptions,
    pub bio_base_url: String,
    pub http: Client,
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    // Leptos config
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;

    // state
    let bio_base_url =
        std::env::var("BIO_URL").unwrap_or_else(|_| "http://bio-service:8000".into());

    let state = AppState {
        leptos_options: leptos_options.clone(),
        bio_base_url,
        http: Client::new(),
    };

    // Leptos routes
    let leptos_routes = generate_route_list(App);

    let api_router = crate::server::bio::router();

    let app = Router::new()
        .merge(api_router)
        .leptos_routes(&leptos_options, routes, App)
        .with_state(leptos_options.clone());

    println!("listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[cfg(not(feature = "ssr"))]
fn main() {}
