// src/app.rs
use leptos::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <main class="p-4">
                <h1>"PLASMIC feat. Genome"</h1>
                <nav class="mt-4 mb-8 flex gap-4">
                    <A href="/">"Home"</A>
                    <A href="/labs">"Labs"</A>
                </nav>

                <Routes>
                    <Route path="/" view=Home />
                    <Route path="/labs" view=LabsPage />
                </Routes>
            </main>
        </Router>
    }
}

// まずはここにベタ書きでOK（あとで ui モジュールに分離）
#[component]
fn Home() -> impl IntoView {
    view! { <div>"ダッシュボード（後でDB / Llama ステータスを出す）"</div> }
}

#[component]
fn LabsPage() -> impl IntoView {
    view! { <div>"ラボ画面（ゲノム実験UI置き場）"</div> }
}
