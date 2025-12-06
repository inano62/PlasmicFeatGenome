use leptos::*;
use leptos_router::*;

use crate::ui::{Home, LabsPage};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <main>
                <h1>"PLASMIC feat. Genome"</h1>
                <nav>
                    <A href="/">"Home"</A>
                    " / "
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
