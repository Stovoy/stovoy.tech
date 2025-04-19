// Compile frontend only for the browser (wasm32 target). Building on the host
// is skipped, avoiding extra dependencies in CLI/CI builds.

#![cfg(target_arch = "wasm32")]

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use web_sys::window;

mod chat;
use chat::Chat;
mod theme;

use theme::{apply_class, current_pref, set_pref};

#[function_component(App)]
fn app() -> Html {
    // Theme state
    let is_dark = use_state(|| current_pref());

    // Ensure class applied on mount / when toggled
    {
        let is_dark = is_dark.clone();
        use_effect_with_deps(
            move |dark| {
                apply_class(**dark);
                || {}
            },
            is_dark.clone(),
        );
    }

    let toggle_theme = {
        let is_dark = is_dark.clone();
        Callback::from(move |_| {
            let new_val = !*is_dark;
            set_pref(new_val);
            is_dark.set(new_val);
        })
    };
    // Determine path to decide what to render.
    let pathname = window()
        .and_then(|w| w.location().pathname().ok())
        .unwrap_or_default();

    let main_content = if pathname == "/game/arena" {
        html! { <Chat /> }
    } else {
        html! {
            <div class="text-center mt-4">
                <h1 class="text-3xl font-bold">{"Stovoy.tech reboot 🚀"}</h1>
                <p class="mt-2 text-gray-600 dark:text-gray-300">{"Hello from Yew + Trunk!"}</p>
            </div>
        }
    };

    html! {
        <>
            <button onclick={toggle_theme} class="fixed bottom-4 right-4 bg-gray-200 dark:bg-gray-700 text-sm px-3 py-1 rounded shadow">
                { if *is_dark { "Light" } else { "Dark" } }
            </button>
            { main_content }
        </>
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}
