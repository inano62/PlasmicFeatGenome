use leptos::*;
use leptos_router::*;

use crate::components::foxp2::Foxp2Demo;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <main>
                <h1>"PLASMIC feat. Genome [BUILD-TEST-0102]"</h1>
                <Routes>
                    <Route path="/" view=Home />
                    <Route path="/foxp2" view=Foxp2Page />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <div>
            <A href="/foxp2">"FOXP2"</A>
        </div>
    }
}

#[component]
fn Foxp2Page() -> impl IntoView {
    view! {
        <main>
            <Foxp2Demo/>
        </main>
    }
}
