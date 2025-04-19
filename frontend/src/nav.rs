#![cfg(target_arch = "wasm32")]

use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let open = use_state(|| false);
    // Obtain the navigation handle and make a small helper that clones the
    // handle each time so that we do not move it into multiple callbacks –
    // this eliminates the previous `use of moved value` and `not Copy`
    // compilation errors.
    let navigator = use_navigator().unwrap();

    let make_link_cb = move |route: Route, open: UseStateHandle<bool>| {
        let nav = navigator.clone();
        Callback::from(move |_| {
            nav.push(&route);
            open.set(false);
        })
    };

    html! {
        <nav class="bg-gray-200 dark:bg-gray-800 p-4 shadow">
            <div class="flex items-center justify-between max-w-4xl mx-auto">
                <span class="font-semibold text-lg">{"stovoy.tech"}</span>

                <button class="md:hidden" onclick={{
                    let open = open.clone();
                    Callback::from(move |_| open.set(!*open))
                }}>
                    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path></svg>
                </button>

                <div class="hidden md:flex gap-4">
                    <button class="hover:underline" onclick={make_link_cb(Route::Home, open.clone())}>{"Home"}</button>
                    <button class="hover:underline" onclick={make_link_cb(Route::Arena, open.clone())}>{"Arena"}</button>
                    <button class="hover:underline" onclick={make_link_cb(Route::Snake, open.clone())}>{"Snake"}</button>
                </div>
            </div>

            <div class={classes!("md:hidden", if *open { "block" } else { "hidden" })}>
                <div class="px-2 pt-2 pb-4 space-y-1">
                    <button class="block w-full text-left hover:underline" onclick={make_link_cb(Route::Home, open.clone())}>{"Home"}</button>
                    <button class="block w-full text-left hover:underline" onclick={make_link_cb(Route::Arena, open.clone())}>{"Arena"}</button>
                    <button class="block w-full text-left hover:underline" onclick={make_link_cb(Route::Snake, open.clone())}>{"Snake"}</button>
                </div>
            </div>
        </nav>
    }
}
