// Compile frontend only for the browser (wasm32 target). Building on the host
// is skipped, avoiding extra dependencies in CLI/CI builds.

#![cfg(target_arch = "wasm32")]

use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod routes;
use routes::Route;

mod nav;
use nav::Navbar;

// Placeholder chat component to unblock compilation while the real
// WebSocket‑based implementation is being reworked.
#[function_component(Chat)]
fn chat_placeholder() -> Html {
    html! { <p>{"Chat will be available soon."}</p> }
}
mod theme;

use theme::{apply_class, current_pref, set_pref};

#[function_component(App)]
fn app() -> Html {
    // Theme state
    let is_dark = use_state(|| current_pref());

    // Ensure class applied on mount / when toggled
    {
        let is_dark = is_dark.clone();
        use_effect_with(
            is_dark.clone(),
            move |dark| {
                apply_class(**dark);
                || {}
            },
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
    use yew_router::prelude::*;

    let switch = Callback::from(move |route: Route| match route {
        Route::Home => html! {
            <div class="text-center mt-4">
                <h1 class="text-3xl font-bold">{"Stovoy.tech reboot 🚀"}</h1>
                <p class="mt-2 text-gray-600 dark:text-gray-300">{"Hello from Yew + Trunk!"}</p>
            </div>
        },
        Route::Arena => html! { <Chat /> },
        Route::Snake => html! { <div class="text-center mt-8">{"Snake v2 coming soon…"}</div> },
        Route::NotFound => html! { <div class="text-center mt-8">{"404"}</div> },
    });

    html! {
        <BrowserRouter>
            <Navbar />
            <button onclick={toggle_theme} class="fixed bottom-4 right-4 bg-gray-200 dark:bg-gray-700 text-sm px-3 py-1 rounded shadow">
                { if *is_dark { "Light" } else { "Dark" } }
            </button>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}
