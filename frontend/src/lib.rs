// Compile frontend only for the browser (wasm32 target). Building on the host
// is skipped, avoiding extra dependencies in CLI/CI builds.

#![cfg(target_arch = "wasm32")]

use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="text-center mt-4">
            <h1 class="text-3xl font-bold">{"Stovoy.tech reboot 🚀"}</h1>
            <p class="mt-2 text-gray-600">{"Hello from Yew + Trunk!"}</p>
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}
