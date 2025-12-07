use leptos::*;
use crate::server::llama::LlamaAsk;

#[component]
pub fn LabsPage() -> impl IntoView {
    let (input, set_input) = create_signal(String::new());

    // ★ server_fn を呼ぶ action
    let ask_action = create_server_action::<LlamaAsk>();

    view! {
        <h1>Llama Labs</h1>

        <textarea
            prop:value=input
            on:input=move |ev| set_input(event_target_value(&ev))
        />

        <button
            on:click=move |_| {
                ask_action.dispatch(LlamaAsk {
                    prompt: input.get()
                });
            }
        >
            "送信"
        </button>

        <div>
            {
                move || match ask_action.value().get() {
                    Some(Ok(res)) => view! { <p>{res}</p> }.into_view(),
                    Some(Err(e))  => view! { <p>Error: {e.to_string()}</p> }.into_view(),
                    None => view! { <p>まだ送信していません</p> }.into_view(),
                }
            }
        </div>
    }
}
