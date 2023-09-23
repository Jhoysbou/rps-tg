use std::time::{Duration, Instant};

use actix::{Actor, Addr};
use actix_web_actors::ws::WebsocketContext;
use uuid::Uuid;

use crate::server::Server;

const PING_INTERVAL: Duration = Duration::from_secs(5);
const TIMEOUT: Duration = Duration::from_secs(10);

pub struct Connection {
    id: Uuid,
    server: Addr<Server>,
    last_ping: Instant,
}

impl Actor for Connection {
    type Context = WebsocketContext<Self>;
}
