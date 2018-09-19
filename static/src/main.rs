#![recursion_limit = "128"]

#[macro_use]
extern crate stdweb;

use stdweb::traits::*;
use stdweb::Value;
use stdweb::unstable::TryInto;
use stdweb::web::{
    document,
    Element,
    window,
};

struct Renderer {
    color: Color,
    canvas: Option<Element>,
    context: Option<Value>,
}

struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Renderer {
    fn initialize(&mut self) {
        let canvas: Element = document().query_selector("#canvas").unwrap().unwrap();
        let context = js! {
            return @{&canvas}.getContext("2d");
        };

        self.canvas = Some(canvas);
        self.context = Some(context);
    }

    fn step(mut self, timestamp: f64) {
        let radius = 100.0;

        let width: f64 = js! { return @{&self.canvas}.width }.try_into().unwrap();
        let height: f64 = js! { return @{&self.canvas}.height }.try_into().unwrap();

        let t = ((timestamp / 1000.0).sin() + 1.0) / 2.0;
        let x = radius / 2.0 + t * (width - radius);
        let y = radius / 2.0 + height / 4.0 + t * (height / 2.0 - radius);

        let color = format!("rgb({}, {}, {})", self.color.r, self.color.g, self.color.b).to_owned();

        self.color.r += 1;
        self.color.g += 2;
        self.color.b += 3;

        js! {
            @{&self.context}.clearRect(0, 0, @{width}, @{height});

            @{&self.context}.beginPath();
            @{&self.context}.arc(@{x}, @{y}, @{radius}, 0, 2 * Math.PI, false);
            @{&self.context}.fillStyle = @{&color};
            @{&self.context}.fill();
            @{&self.context}.lineWidth = 5;
            @{&self.context}.strokeStyle = @{&color};
            @{&self.context}.stroke();
        }

        window().request_animation_frame(|ts: f64| { self.step(ts) });
    }
}


fn main() {
    stdweb::initialize();

    let mut renderer = Renderer { color: Color { r: 0, g: 0, b: 0 }, canvas: None, context: None };
    renderer.initialize();
    window().request_animation_frame(|ts: f64| { renderer.step(ts) });
    stdweb::event_loop();
}
