#![cfg(target_arch = "wasm32")]

use gloo::timers::callback::Interval;
use rand::Rng;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::prelude::*;

/// Size of the logical board (cells per axis).
const GRID: i32 = 20;
/// Pixel size of each cell.
const CELL: i32 = 20;
/// Width / height of the canvas in pixels.
const CANVAS: i32 = GRID * CELL;

/// Direction the snake is currently heading.
#[derive(Copy, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn delta(self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

/// Simple fully‑client‑side snake game rendered to a `<canvas>` element.
#[function_component(SnakeGame)]
pub fn snake_game() -> Html {
    // Canvas reference so we can access the 2D context.
    let canvas_ref: NodeRef = use_node_ref();

    // Mutable game state stored behind `use_mut_ref` so the interval can mutate
    // it without triggering a re‑render on every frame. A render is only
    // needed when the component first mounts; drawing happens directly to the
    // canvas afterwards.
    #[derive(Clone)]
    struct GameState {
        snake: Vec<(i32, i32)>,
        direction: Direction,
        next_direction: Direction,
        food: (i32, i32),
        alive: bool,
    }

    let state = use_mut_ref(|| GameState {
        snake: vec![(GRID / 2, GRID / 2)],
        direction: Direction::Right,
        next_direction: Direction::Right,
        food: (5, 5),
        alive: true,
    });

    // Key handling – updates `next_direction` so the snake turns at most once
    // per tick and never directly reverses.
    {
        let state = state.clone();
        use_effect(move || {
            let listener = gloo::events::EventListener::new(&gloo::utils::window(), "keydown", move |evt| {
                let evt: web_sys::KeyboardEvent = evt.dyn_ref::<web_sys::KeyboardEvent>().unwrap().clone();
                let new_direction = match evt.key().as_str() {
                    "ArrowUp" | "w" | "W" => Some(Direction::Up),
                    "ArrowDown" | "s" | "S" => Some(Direction::Down),
                    "ArrowLeft" | "a" | "A" => Some(Direction::Left),
                    "ArrowRight" | "d" | "D" => Some(Direction::Right),
                    _ => None,
                };
                if let Some(dir) = new_direction {
                    let mut st = state.borrow_mut();
                    // Prevent reversing directly into yourself.
                    let opposite = match st.direction {
                        Direction::Up => Direction::Down,
                        Direction::Down => Direction::Up,
                        Direction::Left => Direction::Right,
                        Direction::Right => Direction::Left,
                    };
                    if dir != opposite {
                        st.next_direction = dir;
                    }
                }
            });
            // Cleanup listener when component unmounts.
            move || drop(listener)
        });
    }

    // Main game loop – 10 FPS.
    {
        let state = state.clone();
        let canvas_ref = canvas_ref.clone();
        use_effect_with(
            (),
            move |_| {
                // Acquire 2d context once.
                let context: CanvasRenderingContext2d = canvas_ref
                    .cast::<HtmlCanvasElement>()
                    .expect("canvas not mounted yet")
                    .get_context("2d")
                    .unwrap()
                    .unwrap()
                    .dyn_into()
                    .unwrap();

                // Helper to (re)draw the entire board.
                let mut draw = move |st: &GameState| {
                    // Clear background.
                    context.set_fill_style(&wasm_bindgen::JsValue::from_str("#f3f4f6"));
                    context.fill_rect(0.0, 0.0, CANVAS as f64, CANVAS as f64);

                    // Draw food.
                    context.set_fill_style(&wasm_bindgen::JsValue::from_str("tomato"));
                    context.fill_rect(
                        (st.food.0 * CELL) as f64,
                        (st.food.1 * CELL) as f64,
                        CELL as f64,
                        CELL as f64,
                    );

                    // Draw snake – color‑cycle for extra ✨.
                    let (r, g, b) = {
                        let t = js_sys::Date::now() as f64 / 100.0;
                        (
                            (t.sin() * 127.0 + 128.0) as u8,
                            (t.cos() * 127.0 + 128.0) as u8,
                            200u8,
                        )
                    };
                    context.set_fill_style(&wasm_bindgen::JsValue::from_str(&format!("rgb({r},{g},{b})")));
                    for (x, y) in &st.snake {
                        context.fill_rect(
                            (*x * CELL) as f64,
                            (*y * CELL) as f64,
                            CELL as f64,
                            CELL as f64,
                        );
                    }
                };

                // Initial draw.
                draw(&state.borrow());

                // Advance the simulation at a fixed interval.
                let interval = Interval::new(100, move || {
                    {
                        let mut st = state.borrow_mut();
                        if !st.alive {
                            // Reset after death.
                            *st = GameState {
                                snake: vec![(GRID / 2, GRID / 2)],
                                direction: Direction::Right,
                                next_direction: Direction::Right,
                                food: (rand::thread_rng().gen_range(0..GRID), rand::thread_rng().gen_range(0..GRID)),
                                alive: true,
                            };
                        }

                        st.direction = st.next_direction;
                        let (dx, dy) = st.direction.delta();
                        let mut new_head = *st.snake.first().unwrap();
                        new_head.0 += dx;
                        new_head.1 += dy;

                        // Collision with walls.
                        if new_head.0 < 0 || new_head.0 >= GRID || new_head.1 < 0 || new_head.1 >= GRID {
                            st.alive = false;
                        }

                        // Collision with self.
                        if st.snake.contains(&new_head) {
                            st.alive = false;
                        }

                        if !st.alive {
                            return;
                        }

                        // Move.
                        st.snake.insert(0, new_head);
                        if new_head == st.food {
                            // Spawn new food.
                            let mut rng = rand::thread_rng();
                            loop {
                                let f = (rng.gen_range(0..GRID), rng.gen_range(0..GRID));
                                if !st.snake.contains(&f) {
                                    st.food = f;
                                    break;
                                }
                            }
                        } else {
                            // Remove tail.
                            st.snake.pop();
                        }
                    }

                    // Redraw with updated state.
                    draw(&state.borrow());
                });

                // Cleanup interval on component unmount.
                move || drop(interval)
            }
        );
    }

    html! {
        <div class="flex flex-col items-center gap-4">
            <canvas
                ref={canvas_ref}
                width={CANVAS.to_string()}
                height={CANVAS.to_string()}
                class="border-4 border-primary shadow-lg rounded bg-gray-100 dark:bg-gray-900"
            />

            <p class="opacity-70 text-sm text-center max-w-sm">
                {"Use WASD or arrow keys. Collide with yourself or a wall to restart. "}
                {"Colors gently shift over time for some extra pizzazz."}
            </p>
        </div>
    }
}
