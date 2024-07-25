use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use reqwest::Client;

#[function_component(ProposalForm)]
pub fn proposal_form() -> Html {
    let description = use_state(|| "".to_string());

    let oninput = {
        let description = description.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            description.set(input.value());
        })
    };

    let onclick = {
        let description = description.clone();
        Callback::from(move |_| {
            let description = description.clone();
            spawn_local(async move {
                let client = Client::new();
                let response = client.post("http://localhost:8000/create_proposal")
                    .json(&serde_json::json!({ "description": (*description).clone() }))
                    .send()
                    .await
                    .unwrap();
                let proposal_id: serde_json::Value = response.json().await.unwrap();
                web_sys::console::log_1(&format!("Proposition créée avec l'ID: {}", proposal_id["proposal_id"]).into());
            });
        })
    };

    html! {
        <div>
            <input type="text" {oninput} placeholder="Description de la proposition" />
            <button {onclick}>{ "Créer une proposition" }</button>
        </div>
    }
}

