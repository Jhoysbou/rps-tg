use std::time::{Duration, Instant};

use actix::{Actor, ActorContext, Addr, AsyncContext, Handler, StreamHandler};
use actix_web_actors::ws::{self, CloseCode, CloseReason, WebsocketContext};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::messages::Close;
use crate::{messages::Connect, server::memory_server::Server, types::UserId};

const PING_INTERVAL: Duration = Duration::from_secs(5);

const TIMEOUT: Duration = Duration::from_secs(10);

pub struct Connection {
    id: Uuid,
    user_id: UserId,
    server: Addr<Server>,
    last_ping: Instant,
}

#[derive(Serialize, Deserialize, Debug)]
enum WSMessageType {
    ConfirmConnect,
}

#[derive(Serialize, Debug)]
struct WSMessage<T>
where
    T: Serialize,
{
    msg_type: WSMessageType,
    data: T,
}

impl Connection {
    fn send_message<T: Serialize>(&self, msg: WSMessage<T>, ctx: &mut ws::WebsocketContext<Self>) {
        match serde_json::to_string(&msg) {
            Ok(txt) => ctx.text(txt),
            Err(err) => log::error!("Couldn't serialize a message: {}", err),
        }
    }

    fn ping(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(PING_INTERVAL, |actor, context| {
            if Instant::now().duration_since(actor.last_ping) > TIMEOUT {
                log::info!("Connection timout for user {}", actor.user_id);
                context.stop();
                return;
            }
            context.ping(b"ping");
        });
    }
}

impl Actor for Connection {
    type Context = WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.ping(ctx);
        let addr = ctx.address();

        self.server.do_send(Connect {
            connection: addr,
            user_id: self.user_id,
        });

        self.send_message(
            WSMessage {
                msg_type: WSMessageType::ConfirmConnect,
                data: "Connection established".to_owned(),
            },
            ctx,
        )
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> actix::Running {
        actix::Running::Stop
    }
}

impl Handler<Close> for Connection {
    type Result = ();

    fn handle(&mut self, msg: Close, ctx: &mut Self::Context) -> Self::Result {
        ctx.close(Some(CloseReason {
            code: CloseCode::Normal,
            description: Some(msg.reason),
        }));
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Connection {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        if msg.is_err() {
            log::error!("There is an error with ws message: {}", msg.err().unwrap());
            return;
        }

        match msg.unwrap() {
            ws::Message::Text(msg) => {
                // let message: WSMessage = serde_json::from_str(&msg);
            }

            // Ignore for now
            ws::Message::Binary(_) => (),
            // Ignore for now
            ws::Message::Continuation(_) => (),

            ws::Message::Ping(_) => {
                self.last_ping = Instant::now();
                ctx.pong(b"pong");
            }
            ws::Message::Pong(_) => self.last_ping = Instant::now(),
            // TODO notify the server
            ws::Message::Close(msg) => ctx.close(msg),
            ws::Message::Nop => (),
        }
    }
}
