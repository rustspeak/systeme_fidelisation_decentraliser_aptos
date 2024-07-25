use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use reqwest::Client;

#[function_component(AccountForm)]
pub fn account_form() -> Html {
    let onclick = Callback::from(move |_| {
        spawn_local(async move {
            let client = Client::new();
            let response = client.post("http://localhost:8000/create_account")
                .send()
                .await
                .unwrap();
            let account_info: serde_json::Value = response.json().await.unwrap();
            web_sys::console::log_1(&format!("Adresse du compte: {}", account_info["address"]).into());
        });
    });

    html! {
        <div>
            <button {onclick}>{ "Créer un compte" }</button>
        </div>
    }
}
