use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use reqwest::Client;

#[function_component(RewardDisplay)]
pub fn reward_display() -> Html {
    let account_address = use_state(|| "".to_string());
    let rewards = use_state(|| 0);

    let oninput = {
        let account_address = account_address.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            account_address.set(input.value());
        })
    };

    let onclick = {
        let account_address = account_address.clone();
        let rewards = rewards.clone();
        Callback::from(move |_| {
            let account_address = account_address.clone();
            spawn_local(async move {
                let client = Client::new();
                let response = client.get(format!("http://localhost:8000/get_rewards?account_address={}", *account_address))
                    .send()
                    .await
                    .unwrap();
                let reward_info: serde_json::Value = response.json().await.unwrap();
                rewards.set(reward_info["total_rewards"].as_u64().unwrap());
            });
        })
    };

    html! {
        <div>
            <input type="text" {oninput} placeholder="Adresse du compte" />
            <button {onclick}>{ "Voir les récompenses" }</button>
            <p>{ format!("Total des récompenses : {}", *rewards) }</p>
        </div>
    }
}