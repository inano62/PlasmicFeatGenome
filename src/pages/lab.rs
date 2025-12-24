
#[component]
fn Lab() -> impl IntoView {
    let (input, set_input) = create_signal(String::new());

    // server_fn を叩く Action
    let ask_action = create_server_action::<LlamaAsk>();

    view! {
        <section>
            <h2 class="mb-2">"Llama Labs"</h2>

            <textarea
                rows="3"
                class="w-full border p-2"
                prop:value=move || input.get()
                on:input=move |ev| set_input.set(event_target_value(&ev))
            />

            <button
                class="mt-2 px-4 py-1 border rounded"
                on:click=move |_| {
                    ask_action.dispatch(LlamaAsk {
                        prompt: input.get()
                    });
                }
            >
                "送信"
            </button>

            <div class="mt-4">
                {move || match ask_action.value().get() {
                    None => view! { <p>"まだ送信していません"</p> }.into_any(),
                    Some(Ok(text)) if text.is_empty() =>
                        view! { <p>"…" </p> }.into_any(),
                    Some(Ok(text)) =>
                        view! { <pre class="whitespace-pre-wrap">{text}</pre> }.into_any(),
                    Some(Err(e)) =>
                        view! { <p class="text-red-600">"Error: " {e.to_string()}</p> }.into_any(),
                }}
            </div>
        </section>
    }
}
