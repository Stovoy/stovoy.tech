use std::cell::RefCell;
use std::rc::Rc;
use stdweb::traits::*;
use stdweb::web::event::{SocketCloseEvent, SocketErrorEvent, SocketMessageEvent, SocketOpenEvent};
use stdweb::web::set_timeout;
use stdweb::web::{document, window, Element, WebSocket};
use stdweb::unstable::TryInto;
use stdweb::Value;

struct ArenaGame {
    tick_interval: u32,
    websocket: Rc<RefCell<WebSocket>>,
    opened: bool,
    canvas: Element,
    context: Value,
}

impl ArenaGame {
    fn new(websocket: Rc<RefCell<WebSocket>>) -> ArenaGame {
        let canvas: Element = document().query_selector("#canvas").unwrap().unwrap();
        js! {
            @{&canvas}.hidden = false;
        }

        let context = js! {
            return @{&canvas}.getContext("2d");
        };

        ArenaGame {
            tick_interval: 100,
            websocket,
            opened: false,
            canvas,
            context,
        }
    }

    fn socket_open(&mut self, event: SocketOpenEvent) {
        self.opened = true;
    }

    fn socket_error(&self, event: SocketErrorEvent) {
        js! { console.log(@{event}) };
    }

    fn socket_close(&self, event: SocketCloseEvent) {
        js! { console.log(@{event}) };
    }

    fn socket_message(&self, event: SocketMessageEvent) {
        js! { console.log(@{&event.data().into_text().unwrap()}) };
    }

    fn tick(&mut self) {
        if !self.opened {
            return;
        }
        self.websocket.borrow_mut().send_text("hello world");
    }
}

fn tick(arena_game: Rc<RefCell<ArenaGame>>) {
    let tick_interval = {
        let mut arena_game_borrowed = arena_game.borrow_mut();
        arena_game_borrowed.tick();

        arena_game_borrowed.tick_interval
    };
    set_timeout(|| tick(arena_game), tick_interval);
}

pub fn run() {
    let hostname: String = js! { return window.location.hostname }.try_into().unwrap();
    let protocol: String = js! { return window.location.protocol }.try_into().unwrap();

    let protocol = protocol.replace("http", "ws");
    let endpoint = format!("{}//{}/api/game/arena", protocol, hostname);

    let websocket = Rc::new(RefCell::new(WebSocket::new(endpoint.as_ref()).unwrap()));

    let arena_game = Rc::new(RefCell::new(ArenaGame::new(websocket.clone())));

    {
        let arena_game = arena_game.clone();
        let tick_interval = arena_game.borrow().tick_interval;
        set_timeout(|| tick(arena_game), tick_interval);
    }

    {
        let arena_game = arena_game.clone();
        websocket
            .clone()
            .borrow_mut()
            .add_event_listener(move |e: SocketOpenEvent| arena_game.borrow_mut().socket_open(e));
    }

    {
        let arena_game = arena_game.clone();
        websocket
            .clone()
            .borrow_mut()
            .add_event_listener(move |e: SocketErrorEvent| arena_game.borrow_mut().socket_error(e));
    }

    {
        let arena_game = arena_game.clone();
        websocket
            .clone()
            .borrow_mut()
            .add_event_listener(move |e: SocketCloseEvent| arena_game.borrow_mut().socket_close(e));
    }

    {
        let arena_game = arena_game.clone();
        websocket
            .clone()
            .borrow_mut()
            .add_event_listener(move |e: SocketMessageEvent| {
                arena_game.borrow_mut().socket_message(e)
            });
    }
}
