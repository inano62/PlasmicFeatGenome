use leptos::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <main>
                <h1>"PLASMIC feat. Genome"</h1>
                <A href="/test">"test"</A>
                <Routes>
                    <Route path="/test" view=Test />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn Test() -> impl IntoView {
    view! { <div>"ok"</div> }
}
