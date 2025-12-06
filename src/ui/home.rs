// src/ui/home.rs
use leptos::*;

#[component]
fn Home() -> impl IntoView {
    let health = create_resource(|| (), |_| async move { get_health().await });

    view! {
        <div>
            <h2>"System Health"</h2>
            {move || match health.get() {
                None => view! { <p>"チェック中..."</p> }.into_any(),
                Some(Ok(data)) => view! { <pre>{data}</pre> }.into_any(),
                Some(Err(e)) => view! { <p>"Error: " {e.to_string()}</p> }.into_any(),
            }}
        </div>
    }
}
