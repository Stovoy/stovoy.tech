#![recursion_limit = "128"]

#[macro_use]
extern crate stdweb;

use rand::prelude::*;
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
use stdweb::web::set_timeout;
use std::cmp::{min, max};

struct Renderer {
    color: Color,
    color_scalars: (i32, i32, i32),
    pub canvas: Option<Element>,
    pub context: Option<Value>,
}

struct Color {
    r: i32,
    g: i32,
    b: i32,
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer { 
            color: Color { r: 0, g: 0, b: 0 }, 
            color_scalars: (0, 0, 0),
            canvas: None,
            context: None 
        }
    }

    fn initialize(&mut self) {
        let canvas: Element = document().query_selector("#canvas").unwrap().unwrap();
        let context = js! {
            return @{&canvas}.getContext("2d");
        };

        self.canvas = Some(canvas);
        self.context = Some(context);
    }

    pub fn update_color(&mut self) {
        let mut rng = thread_rng();

        if rng.gen_range(0, 20) == 0 {
            self.color_scalars.0 = rng.gen_range(-5, 5);
            self.color_scalars.1 = rng.gen_range(-5, 5);
            self.color_scalars.2 = rng.gen_range(-5, 5);
        }

        self.color.r += self.color_scalars.0;
        self.color.g += self.color_scalars.1;
        self.color.b += self.color_scalars.2;

        self.color.r = max(min(self.color.r, 200), 0);
        self.color.g = max(min(self.color.g, 200), 0);
        self.color.b = max(min(self.color.b, 200), 0);
    }

    pub fn get_color(&self) -> String {
        format!("rgb({}, {}, {})", self.color.r, self.color.g, self.color.b).to_owned()
    }
}

#[derive(PartialEq, Clone)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Direction {
    fn reverse(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
        }
    }
}

#[derive(Copy, Clone)]
enum Entity {
    SnakeBody,
    Food,
    None,
}

struct SnakeGame {
    tick_interval: u32,
    board: [[Entity; 10]; 10],
    snake_direction: Direction,
    new_snake_direction: Direction,
    snake_order: Vec<(usize, usize)>,
    food_count: i32,
    max_food: i32,
}

impl SnakeGame {
    pub fn new() -> SnakeGame {
        let mut snake_game = SnakeGame {
            tick_interval: 250,
            board: [[Entity::None; 10]; 10],
            snake_direction: Direction::Right,
            new_snake_direction: Direction::Right,
            snake_order: Vec::new(),
            food_count: 0,
            max_food: 10,
        };

        snake_game.prepend_snake_body((0, 0));

        snake_game
    }

    fn prepend_snake_body(&mut self, point: (usize, usize)) {
        self.board[point.0][point.1] = Entity::SnakeBody;
        self.snake_order.insert(0, point);
    }

    fn pop_snake_body(&mut self) {
        let point = self.snake_order.pop().unwrap();
        self.board[point.0][point.1] = Entity::None;
    }

    fn step(&mut self, timestamp: f64, renderer: &mut Renderer) {
        renderer.update_color();

        self.draw_on_canvas(&renderer);
    }

    fn key_pressed(&mut self, event: KeyDownEvent) {
        let new_direction = match event.key().as_ref() {
            "w" | "ArrowUp" => Direction::Up,
            "a" | "ArrowLeft" => Direction::Left,
            "s" | "ArrowDown" => Direction::Down,
            "d" | "ArrowRight" => Direction::Right,
            _ => return,
        };

        if (self.snake_order.len() == 1 ||
                new_direction != self.snake_direction.reverse()) {
            self.new_snake_direction = new_direction;
        }

        event.prevent_default();
    }

    fn tick(&mut self) {
        if self.food_count < self.max_food {
            self.generate_food();
        }
        self.snake_direction = self.new_snake_direction.clone();
        self.move_snake();

        // Go faster as time goes on.
        self.tick_interval -= 1;
        self.tick_interval = max(self.tick_interval, 100);
    }

    fn generate_food(&mut self) {
        let mut possible_food_points = Vec::new();

        for x in 0..self.board.len() {
            for y in 0..self.board[0].len() {
                match self.board[x][y] {
                    Entity::None => {
                        possible_food_points.push((x, y));
                    },
                    _ => {},
                }
            }
        }

        let mut rng = thread_rng();
        while self.food_count < self.max_food {
            if possible_food_points.len() == 0 {
                break;
            }
            if rng.gen_range(0, 4) != 0 {
                break;
            }

            let index = rng.gen_range(0, possible_food_points.len());
            let food_spawn_point = possible_food_points.remove(index);
            self.board[food_spawn_point.0][food_spawn_point.1] = Entity::Food;
            self.food_count += 1;
        }
    }

    fn move_snake(&mut self) {
        let mut head_point = self.snake_order[0];

        match self.snake_direction {
            Direction::Up => {
                if head_point.1 != 0 {
                    head_point.1 -= 1;
                } else {
                    self.die();
                    return;
                }
            },
            Direction::Left => {
                if head_point.0 != 0 {
                    head_point.0 -= 1;
                } else {
                    self.die();
                    return;
                }
            },
            Direction::Down => {
                if head_point.1 != self.board[0].len() - 1 {
                    head_point.1 += 1;
                } else {
                    self.die();
                    return;
                }
            },
            Direction::Right => {
                if head_point.0 != self.board.len() - 1 {
                    head_point.0 += 1;
                } else {
                    self.die();
                    return;
                }
            }
        }

        let grow = match self.board[head_point.0][head_point.1] {
            Entity::SnakeBody => {
                self.die();
                return;
            },
            Entity::Food => true,
            Entity::None => false,
        };

        self.prepend_snake_body(head_point);
        if !grow {
            self.pop_snake_body();
        } else {
            self.food_count -= 1;
        }
    }

    fn die(&mut self) {
        self.reset();
    }

    fn reset(&mut self) {
        for x in 0..self.board.len() {
            for y in 0..self.board[0].len() {
                self.board[x][y] = Entity::None;
            }
        }

        self.tick_interval = 250;
        self.snake_direction = Direction::Right;
        self.new_snake_direction = Direction::Right;
        self.snake_order.clear();
        self.prepend_snake_body((0, 0));
        self.food_count = 0;
    }

    fn draw_on_canvas(&self, renderer: &Renderer) {
        let width: f64 = js! { return @{&renderer.canvas}.width }.try_into().unwrap();
        let height: f64 = js! { return @{&renderer.canvas}.height }.try_into().unwrap();

        js! {
            @{&renderer.context}.clearRect(0, 0, @{width}, @{height});
        }

        let size = (width / self.board.len() as f64) as i32;

        let color = renderer.get_color();

        let snake_padding = size / 4;
        let snake_size = size - snake_padding;

        let food_padding = size / 2;
        let food_size = size - food_padding;

        // Draw border.
        js! {
            @{&renderer.context}.beginPath();
            @{&renderer.context}.strokeStyle = "darkred";
            @{&renderer.context}.strokeRect(
                0, 0,
                @{width}, @{height});
        };

        for x in 0..self.board.len() {
            for y in 0..self.board[0].len() {
                let entity = self.board[x][y];

                let x = x as i32;
                let y = y as i32;

                match entity {
                    Entity::SnakeBody => {
                        js! {
                            @{&renderer.context}.beginPath();
                            @{&renderer.context}.fillStyle = @{&color};
                            @{&renderer.context}.fillRect(
                                @{x * size + snake_padding / 2}, @{y * size + snake_padding / 2}, 
                                @{snake_size}, @{snake_size});
                        };
                    },
                    Entity::Food => {
                        js! {
                            @{&renderer.context}.beginPath();
                            @{&renderer.context}.fillStyle = "red";
                            @{&renderer.context}.fillRect(
                                @{x * size + food_padding / 2}, @{y * size + food_padding / 2},
                                @{food_size}, @{food_size});
                        };
                    },
                    Entity::None => {},
                }
            }
        }
    }
}

fn animate(ts: f64, snake_game: Rc<RefCell<SnakeGame>>, mut renderer: Renderer) {
    snake_game.borrow_mut().step(ts, &mut renderer);
    window().request_animation_frame(|ts: f64| animate(ts, snake_game, renderer));
}

fn tick(snake_game: Rc<RefCell<SnakeGame>>) {
    let tick_interval = {
        let mut snake_game_borrowed = snake_game.borrow_mut();
        snake_game_borrowed.tick();

        snake_game_borrowed.tick_interval
    };
    set_timeout(|| tick(snake_game), tick_interval);
}

fn main() {
    stdweb::initialize();

    let snake_game = Rc::new(RefCell::new(SnakeGame::new()));
    {
        let snake_game = snake_game.clone();
        window().add_event_listener(move |e| snake_game.borrow_mut().key_pressed(e));
    }

    {
        let mut renderer = Renderer::new();
        renderer.initialize();
        let snake_game = snake_game.clone();
        window().request_animation_frame(|ts: f64| animate(ts, snake_game, renderer));
    }

    {
        let snake_game = snake_game.clone();
        let tick_interval = snake_game.borrow().tick_interval;
        set_timeout(|| tick(snake_game), tick_interval);
    }

    stdweb::event_loop();
}
