use std::time::{Duration, Instant};

use actix::{Actor, Addr, AsyncContext, Handler};
use actix_web_actors::ws::{self, CloseCode, CloseReason, WebsocketContext};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    messages::{Close, Connect},
    server::Server,
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
        ctx.text(serde_json::to_string(&msg).unwrap());
    }
}

impl Actor for Connection {
    type Context = WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();

        self.server.do_send(Connect {
            connection: addr,
            user_id: self.user_id,
        });
    }

    fn stopping(&mut self, ctx: &mut Self::Context) -> actix::Running {
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
