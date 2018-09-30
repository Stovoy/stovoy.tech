use actix::*;
use actix_web::*;

pub mod chat;

pub struct ArenaSessionState {
    pub chat_addr: Addr<chat::ChatServer>,
}

pub fn arena_route(req: &HttpRequest<ArenaSessionState>) -> Result<HttpResponse, Error> {
    ws::start(req, ArenaSession { id: 0, name: None })
}

struct ArenaSession {
    id: usize,
    name: Option<String>,
}

impl Actor for ArenaSession {
    type Context = ws::WebsocketContext<Self, ArenaSessionState>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();
        ctx.state()
            .chat_addr
            .send(chat::Connect {
                addr: addr.recipient(),
            }).into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
                    _ => ctx.stop(),
                }
                fut::ok(())
            }).wait(ctx);
    }

    fn stopping(&mut self, ctx: &mut Self::Context) -> Running {
        ctx.state()
            .chat_addr
            .do_send(chat::Disconnect { id: self.id });
        Running::Stop
    }
}

/// Handle messages from chat server, we simply send it to peer websocket
impl Handler<chat::Message> for ArenaSession {
    type Result = ();

    fn handle(&mut self, msg: chat::Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

impl StreamHandler<ws::Message, ws::ProtocolError> for ArenaSession {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        match msg {
            ws::Message::Ping(msg) => ctx.pong(&msg),
            ws::Message::Text(text) => {
                let msg = if let Some(ref name) = self.name {
                    format!("{}: {}", name, text)
                } else {
                    text.to_owned()
                };

                ctx.state()
                    .chat_addr
                    .do_send(chat::ClientMessage { id: self.id, msg });
            }
            ws::Message::Binary(bin) => ctx.binary(bin),
            ws::Message::Close(_) => {
                ctx.stop();
            }
            _ => (),
        }
    }
}
