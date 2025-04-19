#![cfg(target_arch = "wasm32")]

use yew_router::prelude::*;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/game/arena")]
    Arena,
    #[at("/game/snake")]
    Snake,
    #[not_found]
    #[at("/*")]
    NotFound,
}
