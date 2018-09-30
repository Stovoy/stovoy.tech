use ::actix::*;
use ::actix_web::*;

pub struct ArenaWebsocket;

impl Actor for ArenaWebsocket {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<ws::Message, ws::ProtocolError> for ArenaWebsocket {
    fn handle(&mut self, message: ws::Message, context: &mut Self::Context) {
        match message {
            ws::Message::Ping(message) => context.pong(&message),
            ws::Message::Text(text) => context.text(text),
            ws::Message::Binary(bin) => context.binary(bin),
            _ => (),
        }
    }
}
