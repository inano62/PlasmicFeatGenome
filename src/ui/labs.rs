// src/ui/labs.rs
use leptos::*;
use crate::clients::call_llama;

#[component]
pub fn LabsPage() -> impl IntoView {
    let input = create_rw_signal("ACGT".to_string());
    let action = create_server_action::<AnalyzeWithAi>();

    view! {
        <h2>"Genome AI Fusion"</h2>
        <input
            value=input.get()
            on:input=move |ev| input.set(event_target_value(&ev))
        />
        <button
            on:click=move |_| {
                action.dispatch(AnalyzeWithAi {
                    prompt: input.get(),
                })
            }
        >
            "Analyze"
        </button>

        <p>
            {move || {
                match action.value().get() {
                    Some(Ok(msg)) => msg,
                    Some(Err(e)) => format!("Error: {e}"),
                    None => "結果待ち…".to_string(),
                }
            }}
        </p>
    }
}

#[server]
pub async fn analyze_with_ai(prompt: String) -> Result<String, ServerFnError> {
    let output = crate::clients::call_llama(prompt)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    Ok(format!("AI output: {output}"))
}
