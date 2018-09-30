use std::cell::RefCell;
use std::rc::Rc;
use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::event::{
    KeyDownEvent, SocketCloseEvent, SocketErrorEvent, SocketMessageEvent, SocketOpenEvent,
};
use stdweb::web::set_timeout;
use stdweb::web::{document, Element, WebSocket};

struct ArenaGame {
    tick_interval: u32,
    websocket: Rc<RefCell<WebSocket>>,
    opened: bool,
    game_text_box: Element,
    text_input: Element,
}

impl ArenaGame {
    fn new(websocket: Rc<RefCell<WebSocket>>) -> ArenaGame {
        let game_container: Element = document().query_selector("#farming-game").unwrap().unwrap();
        let game_text_box: Element = document()
            .query_selector("#farming-game-text-box")
            .unwrap()
            .unwrap();
        let text_input: Element = document()
            .query_selector("#farming-game-text-input")
            .unwrap()
            .unwrap();

        js! {
            @{&game_container}.hidden = false;
        }

        ArenaGame {
            tick_interval: 1000 / 30,
            websocket,
            opened: false,
            game_text_box,
            text_input,
        }
    }

    fn socket_open(&mut self, _event: SocketOpenEvent) {
        self.opened = true;
    }

    fn socket_error(&self, event: SocketErrorEvent) {
        js! { console.log(@{event}) };
    }

    fn socket_close(&mut self, _event: SocketCloseEvent) {
        self.opened = false;
    }

    fn socket_message(&mut self, event: SocketMessageEvent) {
        let element = document().create_element("div").unwrap();
        element.set_text_content(event.data().into_text().unwrap().as_ref());
        if let Some(child) = self.game_text_box.first_child() {
            self.game_text_box.insert_before(&element, &child);
        } else {
            self.game_text_box.append_child(&element);
        }
    }

    fn key_pressed(&mut self, event: KeyDownEvent) {
        if event.key() != "Enter" {
            return;
        }

        let value: String = js! {
            const value = @{&self.text_input}.value;
            @{&self.text_input}.value = "";
            return value;
        }.try_into()
        .unwrap();
        self.websocket.borrow_mut().send_text(value.as_ref());
    }

    fn tick(&mut self) {
        if !self.opened {
            return;
        }
        // self.websocket.borrow_mut().send_text("hello world");
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
            .add_event_listener(move |e| arena_game.borrow_mut().socket_open(e));
    }

    {
        let arena_game = arena_game.clone();
        websocket
            .clone()
            .borrow_mut()
            .add_event_listener(move |e| arena_game.borrow_mut().socket_error(e));
    }

    {
        let arena_game = arena_game.clone();
        websocket
            .clone()
            .borrow_mut()
            .add_event_listener(move |e| arena_game.borrow_mut().socket_close(e));
    }

    {
        let arena_game = arena_game.clone();
        websocket
            .clone()
            .borrow_mut()
            .add_event_listener(move |e| arena_game.borrow_mut().socket_message(e));
    }

    {
        let arena_game = arena_game.clone();

        let text_input: Element = document()
            .query_selector("#farming-game-text-input")
            .unwrap()
            .unwrap();
        text_input.add_event_listener(move |e| arena_game.borrow_mut().key_pressed(e));
    }
}
