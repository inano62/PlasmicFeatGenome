use leptos::*;
use leptos_router::*;
use crate::clients::call_llama;
use leptos::ServerFnError;

#[component]
pub fn Labs() -> impl IntoView {
    let input = create_rw_signal("ACGT".to_string());
    let action = create_server_action::<AnalyzeWithAi>();

    view! {
        <div>
            <h1>"Genome AI Labs"</h1>

            <input
                value=input.get()
                on:input=move |ev| input.set(event_target_value(&ev))
            />

            <button on:click=move |_| {
                action.dispatch(AnalyzeWithAi {
                    prompt: input.get()
                });
            }>
                "Analyze"
            </button>

            <p>
                {move || {
                    action
                        .value()
                        .get()
                        .as_ref()
                        .map(|res| match res {
                            Ok(msg) => msg.clone(),
                            Err(e) => format!("error: {e}"),
                        })
                        .unwrap_or_else(|| "no result yet".to_string())
                }}
            </p>
        </div>
    }
}

#[server]
pub async fn analyze_with_ai(prompt: String) -> Result<String, ServerFnError> {
    let output = call_llama(prompt)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    Ok(format!("LLAMA: {}", output))
}
