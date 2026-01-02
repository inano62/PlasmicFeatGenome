use leptos::*;
use serde::Deserialize;
use crate::shared::bio_types::AnalyzeReq;

#[derive(Deserialize, Clone)]
struct AnalyzeRes {
    record_id: String,
    length_nt: i32,
    gc_percent: f64,
    facts: String,
    summary: Option<String>,
}

#[component]
pub fn Foxp2Demo() -> impl IntoView {
    let (fasta, set_fasta) = create_signal(String::new());
    let (result, set_result) = create_signal::<Option<AnalyzeRes>>(None);
    let (_err, set_err) = create_signal::<Option<String>>(None);

    let on_click = move |_| {
        let fasta_val = fasta.get();
        spawn_local(async move {
            let body = AnalyzeReq {
                fasta: fasta_val,
                prompt: Some("事実だけを短く説明して。".into()),
                max_tokens: Some(256),
            };

            let resp = reqwasm::http::Request::post("/api/bio/foxp2/analyze")
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&body).unwrap())
                .send()
                .await;

            match resp {
                Ok(r) if r.ok() => {
                    let json = r.json::<AnalyzeRes>().await;
                    if let Ok(v) = json {
                        set_result.set(Some(v));
                    }
                }
                Ok(r) => set_err.set(Some(format!("http {}", r.status()))),
                Err(e) => set_err.set(Some(format!("{e:?}"))),
            }
        });
    };

    view! {
        <div>
            <h2>"FOXP2 demo"</h2>
            <textarea
                rows="10"
                on:input=move |ev| set_fasta.set(event_target_value(&ev))
            />
            <button on:click=on_click>"解析"</button>

            {move || result.get().map(|r| view!{
                <pre>{r.summary.unwrap_or_default()}</pre>
            })}
        </div>
    }
}
