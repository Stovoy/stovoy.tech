use actix::prelude::*;
use std::collections::HashMap;

#[derive(Message)]
pub struct Message(pub String);

#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<Message>,
}

#[derive(Message)]
pub struct Disconnect {
    pub id: usize,
}

#[derive(Message)]
pub struct ClientMessage {
    pub id: usize,
    pub msg: String,
}

pub struct ChatServer {
    sessions: HashMap<usize, Recipient<Message>>,
    current_id: usize,
}

impl Default for ChatServer {
    fn default() -> ChatServer {
        ChatServer {
            sessions: HashMap::new(),
            current_id: 0,
        }
    }
}

impl ChatServer {
    fn send_message(&self, message: &str) {
        for id in self.sessions.keys() {
            if let Some(addr) = self.sessions.get(id) {
                let _ = addr.do_send(Message(message.to_owned()));
            }
        }
    }
}

impl Actor for ChatServer {
    type Context = Context<Self>;
}

impl Handler<Connect> for ChatServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        println!("Someone joined");

        self.send_message("Someone joined");

        let id = self.current_id;

        self.current_id += 1;

        self.sessions.insert(id, msg.addr);

        id
    }
}

impl Handler<Disconnect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        println!("Someone disconnected");

        self.sessions.remove(&msg.id);

        self.send_message("Someone disconnected");

        self.current_id -= 1;
    }
}

impl Handler<ClientMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>) {
        let text = msg.msg.as_str();

        println!("Received message: {}", text);

        self.send_message(text);
    }
}
