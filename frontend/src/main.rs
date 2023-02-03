use gloo::{console::log, dialogs::alert};
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use serde_json::json;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Deserialize, Serialize)]
struct Response {
    info: String,
    message: String,
    token: String,
}

#[derive(Deserialize, Serialize)]
struct LoginData {
    data: Response,
}

async fn login(username: String, password: String) -> Response {
    let data = json!({
        "username": username,
        "password": password,
    });

    let response = Request::post("http://localhost:8090/user/encode-token")
        .header("content-type", "application/json")
        .body(data.to_string())
        .send()
        .await
        .unwrap()
        .json::<LoginData>()
        .await
        .unwrap();
    response.data
}

#[function_component]
fn MyComponent() -> Html {
    let name_input_value_handle = use_state(String::default);
    let name_input_value = (*name_input_value_handle).clone();
    let pass_input_value_handle = use_state(String::default);
    let pass_input_value = (*pass_input_value_handle).clone();

    let on_submit = {
        spawn_local(async move {
            let res = login(name_input_value.clone(), pass_input_value.clone()).await;
            alert(&res.token);
        })
    };

    let on_name_change = {
        let name_input_value_handle = name_input_value_handle.clone();

        Callback::from(move |e: Event| {
            let input = e.target_dyn_into::<HtmlInputElement>();

            if let Some(input) = input {
                name_input_value_handle.set(input.value().clone());
                log!(input.value());
            }
        })
    };

    let on_dangerous_change = Callback::from(move |e: Event| {
        // You must KNOW target is a HtmlInputElement, otherwise
        // the call to value would be Undefined Behaviour (UB).
        pass_input_value_handle.set(e.target_unchecked_into::<HtmlInputElement>().value());
    });

    html! {
        <>
            <label for="cautious-input">
                { "My cautious input:" }
                <input onchange={on_name_change}
                    id="cautious-input"
                    type="text"
                />
            </label>
            <label for="dangerous-input">
                { "My dangerous input:" }
                <input onchange={on_dangerous_change}
                    id="dangerous-input"
                    type="text"
                />
            </label>
            <button onclick={Callback::from(move |_| on_submit)}>{"Click Me!"}</button>
        </>
    }
}

fn main() {
    yew::Renderer::<MyComponent>::new().render();
}
