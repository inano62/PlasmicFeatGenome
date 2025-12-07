mod server;
use axum::Router;
use leptos::*;
use leptos_axum::{generate_route_list, LeptosRoutes};

mod app;
use crate::app::App;

#[tokio::main]
async fn main() {
    let conf = get_configuration(Some("Cargo.toml")).await.unwrap();
    let addr = conf.leptos_options.site_addr;

    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, App)
        .with_state(leptos_options);

    println!("listening on http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
