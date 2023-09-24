use std::time::{Duration, Instant};

use actix::{
    fut, Actor, ActorContext, ActorFutureExt, Addr, AsyncContext, ContextFutureSpawner, Handler,
    StreamHandler, WrapFuture,
};
use actix_web_actors::ws::{self, CloseCode, CloseReason, WebsocketContext};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{
    client_messages::{ConfirmConnectPayload, IncomingClientMessage, OutgoingClientMessage},
    messages::{Close, SendClientMessage},
};
use crate::{
    server::{
        memory_server::Server,
        messages::{AttachConnection, ProcessClientMessage},
    },
    transport::client_messages::ErrorPayload,
    types::UserId,
};

const PING_INTERVAL: Duration = Duration::from_secs(5);

const TIMEOUT: Duration = Duration::from_secs(10);

pub struct Connection {
    id: Uuid,
    user_id: UserId,
    server: Addr<Server>,
    last_ping: Instant,
}

impl Connection {
    fn send_message(&self, msg: OutgoingClientMessage, ctx: &mut ws::WebsocketContext<Self>) {
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

        self.server.do_send(AttachConnection {
            connection: addr,
            user_id: self.user_id,
        });

        self.send_message(
            OutgoingClientMessage::ConfirmConnect(ConfirmConnectPayload {
                message: "Connection established".to_owned(),
            }),
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

impl Handler<SendClientMessage> for Connection {
    type Result = ();

    fn handle(&mut self, msg: SendClientMessage, ctx: &mut Self::Context) -> Self::Result {
        self.send_message(msg.message, ctx);
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
                if let Ok(message) = serde_json::from_str::<IncomingClientMessage>(&msg) {
                    self.server
                        .send(ProcessClientMessage {
                            message,
                            user_id: self.user_id,
                        })
                        .into_actor(self)
                        .then(|res, conn, ctx| {
                            if let Err(err) = res {
                                log::error!("Couldn't send message to actor: {}", err);
                                return fut::ready(());
                            }
                            match res.unwrap() {
                                Ok(result) => {
                                    conn.send_message(OutgoingClientMessage::from(result), ctx);
                                }
                                Err(err) => {
                                    log::error!("Process message error: {}", err);
                                    conn.send_message(
                                        OutgoingClientMessage::Error(ErrorPayload {
                                            message: "Internal error: Couldn't process event"
                                                .to_owned(),
                                        }),
                                        ctx,
                                    );
                                }
                            }

                            fut::ready(())
                        })
                        .wait(ctx);
                } else {
                    log::error!("Couldn parse message: {}", msg);
                }
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
