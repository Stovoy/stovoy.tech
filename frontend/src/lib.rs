// Compile frontend only for the browser (wasm32 target). Building on the host
// is skipped, avoiding extra dependencies in CLI/CI builds.

#![cfg(target_arch = "wasm32")]

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use web_sys::window;

mod chat;
use chat::Chat;

#[function_component(App)]
fn app() -> Html {
    // Determine path to decide what to render.
    let pathname = window()
        .and_then(|w| w.location().pathname().ok())
        .unwrap_or_default();

    if pathname == "/game/arena" {
        html! { <Chat /> }
    } else {
        html! {
            <div class="text-center mt-4">
                <h1 class="text-3xl font-bold">{"Stovoy.tech reboot 🚀"}</h1>
                <p class="mt-2 text-gray-600">{"Hello from Yew + Trunk!"}</p>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}
