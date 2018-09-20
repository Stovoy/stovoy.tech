#![recursion_limit = "128"]

#[macro_use]
extern crate stdweb;


use std::cell::RefCell;
use std::rc::Rc;
use stdweb::traits::*;
use stdweb::Value;
use stdweb::unstable::TryInto;
use stdweb::web::{
    document,
    Element,
    window,
};
use stdweb::web::event::KeyDownEvent;

struct Renderer {
    color: Color,
    pub canvas: Option<Element>,
    pub context: Option<Value>,
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

    pub fn update_color(&mut self) {
        self.color.r += 1;
        self.color.g += 2;
        self.color.b += 3;
    }

    pub fn get_color(&self) -> String {
        format!("rgb({}, {}, {})", self.color.r, self.color.g, self.color.b).to_owned()
    }
}

struct SnakeGame {
    head: (u32, u32),
    board: [[u32; 5]; 5],
}

enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl SnakeGame {
    pub fn new() -> SnakeGame {
        SnakeGame {
            head: (0, 0),
            board: [[0u32; 5]; 5]
        }
    }

    fn step(&mut self, timestamp: f64, renderer: &mut Renderer) {
        renderer.update_color();

        self.draw_on_canvas(&renderer);
    }

    fn key_pressed(&mut self, event: KeyDownEvent) {
        match event.key().as_ref() {
            "w" | "ArrowUp" => self.move_snake(Direction::Up),
            "a" | "ArrowLeft" => self.move_snake(Direction::Left),
            "s" | "ArrowDown" => self.move_snake(Direction::Down),
            "d" | "ArrowRight" => self.move_snake(Direction::Right),
            _ => return,
        }

        event.prevent_default();
    }

    fn move_snake(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.head.1 -= 1,
            Direction::Left => self.head.0 -= 1,
            Direction::Down => self.head.1 += 1,
            Direction::Right => self.head.0 += 1,
        }
    }

    fn draw_on_canvas(&self, renderer: &Renderer) {
        let width: f64 = js! { return @{&renderer.canvas}.width }.try_into().unwrap();
        let height: f64 = js! { return @{&renderer.canvas}.height }.try_into().unwrap();

        js! {
            @{&renderer.context}.clearRect(0, 0, @{width}, @{height});
        }

        let size = 100;

        let color = renderer.get_color();

        for x in 0..5 {
            for y in 0..5 {
                js! {
                    @{&renderer.context}.beginPath();
                    @{&renderer.context}.rect(@{x * size}, @{y * size}, @{size}, @{size});
                    @{&renderer.context}.lineWidth = 2;
                    @{&renderer.context}.strokeStyle = @{&color};
                    @{&renderer.context}.stroke();
                };
            }
        }

        let snake_head_padding = 10;
        let snake_head_size = size - snake_head_padding;
        js! {
            @{&renderer.context}.beginPath();
            @{&renderer.context}.fillStyle = @{&color};
            @{&renderer.context}.fillRect(
                @{self.head.0 * size + snake_head_padding / 2}, @{self.head.1 * size + snake_head_padding / 2}, 
                @{snake_head_size}, @{snake_head_size});
        };
    }
}

fn animate(ts: f64, snake_game: Rc<RefCell<SnakeGame>>, mut renderer: Renderer) {
    snake_game.borrow_mut().step(ts, &mut renderer);
    window().request_animation_frame(|ts: f64| animate(ts, snake_game, renderer));
}

fn main() {
    stdweb::initialize();

    let mut renderer = Renderer { color: Color { r: 0, g: 0, b: 0 }, canvas: None, context: None };
    renderer.initialize();

    let snake_game = Rc::new(RefCell::new(SnakeGame::new()));
    {
        let snake_game = snake_game.clone();
        window().add_event_listener(move |e| snake_game.borrow_mut().key_pressed(e));
    }

    {
        let snake_game = snake_game.clone();
        window().request_animation_frame(|ts: f64| animate(ts, snake_game, renderer));
    }

    stdweb::event_loop();
}
