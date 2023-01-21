use web_sys::HtmlInputElement;
use yew::prelude::*;

impl Data {}

#[function_component]
fn App() -> Html {
    let input_node_ref = use_node_ref();

    let onchange = {
        let input_node_ref = input_node_ref.clone();

        Callback::from(move |_| {
            if let Some(input) = input_node_ref.cast::<HtmlInputElement>() {
                let input = input.value();

                html! {
                    <code>{input}</code>
                };
            }
        })
    };

    html! {
        <>
            <label for="my-input">
                {"Input: "}
                <input ref={input_node_ref} {onchange} id="my-input" type="text"/>
            </label>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
