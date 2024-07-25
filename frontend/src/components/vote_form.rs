use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use reqwest::Client;

#[function_component(VoteForm)]
pub fn vote_form() -> Html {
    let proposal_id = use_state(|| "".to_string());
    let vote_for = use_state(|| true);

    let oninput_id = {
        let proposal_id = proposal_id.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            proposal_id.set(input.value());
        })
    };

    let oninput_vote = {
        let vote_for = vote_for.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            vote_for.set(input.value() == "true");
        })
    };

    let onclick = {
        let proposal_id = proposal_id.clone();
        let vote_for = vote_for.clone();
        Callback::from(move |_| {
            let proposal_id = proposal_id.clone();
            let vote_for = vote_for.clone();
            spawn_local(async move {
                let client = Client::new();
                let response = client.post("http://localhost:8000/submit_vote")
                    .json(&serde_json::json!({ "proposal_id": (*proposal_id).parse::<u64>().unwrap(), "vote_for": *vote_for }))
                    .send()
                    .await
                    .unwrap();
                let result: serde_json::Value = response.json().await.unwrap();
                web_sys::console::log_1(&format!("Résultat du vote: {}", result["status"]).into());
            });
        })
    };

    html! {
        <div>
            <input type="text" {oninput_id} placeholder="ID de la proposition" />
            <select {oninput_vote}>
                <option value="true">{ "Pour" }</option>
                <option value="false">{ "Contre" }</option>
            </select>
            <button {onclick}>{ "Soumettre un vote" }</button>
        </div>
    }
}