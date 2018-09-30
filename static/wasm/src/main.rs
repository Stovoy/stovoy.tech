#![recursion_limit = "128"]

#[macro_use]
extern crate stdweb;
mod arena;
mod snake;

use stdweb::unstable::TryInto;

fn main() {
    stdweb::initialize();

    let path: String = js! { return window.location.pathname }.try_into().unwrap();

    match path.as_ref() {
        "/game/arena" => arena::run(),
        "/game/snake" => snake::run(),
        _ => {}
    };

    stdweb::event_loop();
}
