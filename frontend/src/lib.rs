#![cfg(target_arch = "wasm32")]

use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod routes;
use routes::Route;

mod snake;
use snake::SnakeGame;


mod home;
use home::Home;

// Removed theme toggle, dark vs light functionality.

#[function_component(App)]
fn app() -> Html {
    // Removed dark theme preference logic.

    use yew_router::prelude::*;

    let switch = Callback::from(move |route: Route| match route {
        Route::Home => html! { <Home /> },
        Route::Snake => html! { <SnakeGame /> },
        Route::NotFound => html! { <div class="text-center mt-8">{"404"}</div> },
    });

    html! {
        <BrowserRouter>
            <div class="font-sans min-h-screen bg-gradient-to-br from-gray-50 to-white text-gray-800 flex flex-col">
                <main class="flex-grow max-w-4xl w-full mx-auto px-4 py-8">
                    <Switch<Route> render={switch.clone()} />
                </main>
            </div>
        </BrowserRouter>
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}
