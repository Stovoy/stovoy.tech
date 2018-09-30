#![recursion_limit = "128"]

#[macro_use]
extern crate stdweb;
mod snake;

fn main() {
    stdweb::initialize();

    let path = js! { return window.location.pathname };
    if path == "/game/snake" {
        snake::run();
    }

    stdweb::event_loop();
}
