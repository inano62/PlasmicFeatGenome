use leptos::*;
use leptos_router::*;

use crate::server::llama::LlamaAsk;
use crate::pages::{Home, LabsPage};
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