#![cfg(target_arch = "wasm32")]

use gloo::timers::callback::Interval;
use rand::Rng;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::prelude::*;

const GRID: i32 = 20;
const CELL: i32 = 20;
const CANVAS: i32 = GRID * CELL;

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

#[function_component(SnakeGame)]
pub fn snake_game() -> Html {
    let canvas_ref: NodeRef = use_node_ref();

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

    {
        let state = state.clone();
        use_effect(move || {
            let listener =
                gloo::events::EventListener::new(&gloo::utils::window(), "keydown", move |evt| {
                    let evt: web_sys::KeyboardEvent =
                        evt.dyn_ref::<web_sys::KeyboardEvent>().unwrap().clone();
                    let new_direction = match evt.key().as_str() {
                        "ArrowUp" | "w" | "W" => Some(Direction::Up),
                        "ArrowDown" | "s" | "S" => Some(Direction::Down),
                        "ArrowLeft" | "a" | "A" => Some(Direction::Left),
                        "ArrowRight" | "d" | "D" => Some(Direction::Right),
                        _ => None,
                    };
                    if let Some(dir) = new_direction {
                        let mut st = state.borrow_mut();
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
            move || drop(listener)
        });
    }

    {
        let state = state.clone();
        let canvas_ref = canvas_ref.clone();
        use_effect_with((), move |_| {
            let context: CanvasRenderingContext2d = canvas_ref
                .cast::<HtmlCanvasElement>()
                .expect("canvas not mounted yet")
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into()
                .unwrap();

            let draw = move |st: &GameState| {
                context.set_fill_style(&wasm_bindgen::JsValue::from_str("#f3f4f6"));
                context.fill_rect(0.0, 0.0, CANVAS as f64, CANVAS as f64);

                context.set_fill_style(&wasm_bindgen::JsValue::from_str("tomato"));
                context.fill_rect(
                    (st.food.0 * CELL) as f64,
                    (st.food.1 * CELL) as f64,
                    CELL as f64,
                    CELL as f64,
                );

                let (r, g, b) = {
                    let t = js_sys::Date::now() as f64 / 100.0;
                    (
                        (t.sin() * 127.0 + 128.0) as u8,
                        (t.cos() * 127.0 + 128.0) as u8,
                        200u8,
                    )
                };
                context.set_fill_style(&wasm_bindgen::JsValue::from_str(&format!(
                    "rgb({r},{g},{b})"
                )));
                for (x, y) in &st.snake {
                    context.fill_rect(
                        (*x * CELL) as f64,
                        (*y * CELL) as f64,
                        CELL as f64,
                        CELL as f64,
                    );
                }
            };

            draw(&state.borrow());

            let interval = Interval::new(100, move || {
                {
                    let mut st = state.borrow_mut();
                    if !st.alive {
                        *st = GameState {
                            snake: vec![(GRID / 2, GRID / 2)],
                            direction: Direction::Right,
                            next_direction: Direction::Right,
                            food: (
                                rand::thread_rng().gen_range(0..GRID),
                                rand::thread_rng().gen_range(0..GRID),
                            ),
                            alive: true,
                        };
                    }

                    st.direction = st.next_direction;
                    let (dx, dy) = st.direction.delta();
                    let mut new_head = *st.snake.first().unwrap();
                    new_head.0 += dx;
                    new_head.1 += dy;

                    if new_head.0 < 0 || new_head.0 >= GRID || new_head.1 < 0 || new_head.1 >= GRID
                    {
                        st.alive = false;
                    }

                    if st.snake.contains(&new_head) {
                        st.alive = false;
                    }

                    if !st.alive {
                        return;
                    }

                    st.snake.insert(0, new_head);
                    if new_head == st.food {
                        let mut rng = rand::thread_rng();
                        loop {
                            let f = (rng.gen_range(0..GRID), rng.gen_range(0..GRID));
                            if !st.snake.contains(&f) {
                                st.food = f;
                                break;
                            }
                        }
                    } else {
                        st.snake.pop();
                    }
                }

                draw(&state.borrow());
            });

            move || drop(interval)
        });
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
