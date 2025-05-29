// Compile frontend only for the browser (wasm32 target). Building on the host
// is skipped, avoiding extra dependencies in CLI/CI builds.

#![cfg(target_arch = "wasm32")]

use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod routes;
use routes::Route;

mod nav;
use nav::Navbar;
mod snake;
use snake::SnakeGame;

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
        use_effect_with(is_dark.clone(), move |dark| {
            apply_class(**dark);
            || {}
        });
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
            <section class="text-center flex flex-col items-center justify-center gap-6 py-12">
                <h1 class="text-5xl md:text-6xl font-extrabold bg-gradient-to-r from-primary to-indigo-600 bg-clip-text text-transparent drop-shadow">{"Stovoy.tech"}</h1>
                <p class="max-w-xl text-lg md:text-xl opacity-80">{"Rust‑powered playground, streaming assorted side‑projects and experiments. Have a poke around – source is only a click away."}</p>

                <div class="flex gap-4 mt-6">
                    <a href="#arena" class="px-6 py-3 rounded-full bg-primary text-white hover:bg-indigo-600 transition-colors shadow">{"Enter Arena"}</a>
                    <a href="#snake" class="px-6 py-3 rounded-full bg-white/10 border border-primary text-primary hover:bg-primary hover:text-white transition-colors shadow">{"Play Snake"}</a>
                </div>
            </section>
        },
        Route::Arena => html! { <Chat /> },
        Route::Snake => html! { <SnakeGame /> },
        Route::NotFound => html! { <div class="text-center mt-8">{"404"}</div> },
    });

    html! {
        <BrowserRouter>
            <div class="font-sans min-h-screen bg-gradient-to-br from-gray-50 to-white dark:from-gray-900 dark:to-gray-800 text-gray-800 dark:text-gray-100 flex flex-col">
                <Navbar />
                <main class="flex-grow max-w-4xl w-full mx-auto px-4 py-8">
                    <Switch<Route> render={switch.clone()} />
                </main>

                <button onclick={toggle_theme} class="fixed bottom-5 right-5 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-700 rounded-full shadow p-2 hover:scale-105 transition-transform">
                    { if *is_dark { "☀️" } else { "🌙" } }
                </button>
            </div>
        </BrowserRouter>
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}
