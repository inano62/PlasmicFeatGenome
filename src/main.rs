// src/main.rs
use axum::Router;
use leptos::*;
use leptos_axum::{generate_route_list, LeptosRoutes};

mod app;
use crate::app::App;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    // Leptos config（Cargo.toml の metadata.leptos）
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;

    // Leptos routes
    let routes = generate_route_list(App);

    // Router を作る
    let app = Router::new()
        .leptos_routes(&leptos_options, routes, App)
        .with_state(leptos_options.clone()); // ← ここ重要

    println!("listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();

    // ★ into_service / into_make_service は使わない
    axum::serve(listener, app)
        .await
        .unwrap();
}

// WASM / hydrate 用（cargo-leptos が bin を要求するため）
#[cfg(not(feature = "ssr"))]
fn main() {}
