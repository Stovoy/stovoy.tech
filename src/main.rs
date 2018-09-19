extern crate ctrlc;
extern crate futures;
extern crate gotham;
extern crate hyper;
extern crate mime;

use gotham::http::response::create_response;
use gotham::state::State;
use hyper::{Response, StatusCode};
use std::process::exit;

pub fn say_hello(state: State) -> (State, Response) {
    let res = create_response(
        &state,
        StatusCode::Ok,
        Some((String::from("Hello World!").into_bytes(), mime::TEXT_PLAIN)),
    );

    (state, res)
}

pub fn main() {
    ctrlc::set_handler(move || {
        println!("SIGTERM or SIGINT detected, exiting.");
        exit(1);
    }).expect("Error setting Ctrl-C handler");

    let addr = "0.0.0.0:80";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, || Ok(say_hello))
}
