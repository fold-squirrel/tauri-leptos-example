use leptos::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Serialize, Deserialize)]
struct GreetArgs {
    name: String,
}

async fn invoke_greet(name: ReadSignal<String>, set_greet_msg: WriteSignal<String>) {
    let msg = invoke("greet", to_value(&GreetArgs { name: name() }).unwrap()).await;
    log(&msg.as_string().unwrap());
    set_greet_msg(msg.as_string().unwrap());
}

#[component]
pub fn TauriApp(cx: Scope) -> Element {
    let (name, set_name) = create_signal(cx, String::new());
    let (greet_msg, set_greet_msg) = create_signal(cx, String::new());

    let greet = create_action(
        cx,
        |name_and_greet_msg_setter: &(ReadSignal<String>, WriteSignal<String>)| {
            let (name, set_greet_msg) = name_and_greet_msg_setter;
            invoke_greet(*name, *set_greet_msg)
        },
    );

    view! {
        cx,
        <div class="container">
            <h1>"Welcome to Tauri!"</h1>

            <div class="row">
                <a
                    href="https://tauri.app"
                    target="_blank">
                    <img
                        src="public/tauri.svg"
                        class="logo tauri"
                        alt="Tauri logo"
                    />
                </a>
                <a
                    href="https://developer.mozilla.org/en-US/docs/Web/JavaScript"
                    target="_blank"
                >
                    <img
                        src="public/leptos_logo.svg"
                        class="logo leptos"
                        alt="JavaScript logo"
                    />
                </a>
            </div>

            <p>"Tauri + Leptos"</p>

            <div class="row">
                <div>
                    <input
                        id="greet-input"
                        on:input=move |ev| {let entered_name = event_target_value(&ev).trim().to_string(); set_name(entered_name) }
                        placeholder="Enter a name..."
                    />
                    <button
                        id="greet-button"
                        on:click=move |_| {if !name().is_empty() { greet.dispatch((name, set_greet_msg)) }}
                    >
                        "Greet"
                    </button>
                </div>
            </div>

            <p id="greet-msg">{move || greet_msg().to_string()}</p>
        </div>
    }
}
