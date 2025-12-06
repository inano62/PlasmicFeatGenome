mod ui;
mod clients;

use axum::Router;
use leptos::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use ui::HomePage;
use ui::LabsPage;

#[tokio::main]
async fn main() {
    // Leptos の設定を Cargo.toml から読む
    let conf = leptos_config::get_configuration(Some("Cargo.toml"))
        .await
        .expect("load leptos config");

    // 0.0.0.0:3000 固定
    let mut leptos_options = conf.leptos_options;
    let addr = "0.0.0.0:3000".parse().unwrap();
    leptos_options.site_addr = addr;

    // App コンポーネントからルート一覧を生成
    let routes = generate_route_list(app::App);

    // Axum に Leptos をマウント
    let app = Router::new()
        .leptos_routes(&leptos_options, routes, app::App)
        .with_state(leptos_options);

    // axum 0.7 流儀で起動
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("listening on http://{addr}");
    axum::serve(listener, app).await.unwrap();
}
