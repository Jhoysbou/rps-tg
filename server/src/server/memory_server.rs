use std::collections::HashMap;

use actix::{Actor, Addr, Context, Handler};

use crate::{
    transport::{messages::Close, ws::Connection},
    types::UserId,
};

use super::messages::Connect;

pub struct Server {
    connections: HashMap<UserId, Addr<Connection>>,
}

impl Actor for Server {
    type Context = Context<Self>;
}

impl Handler<Connect> for Server {
    type Result = ();

    fn handle(&mut self, msg: Connect, _ctx: &mut Self::Context) -> Self::Result {
        if let Some(old_connection) = self.connections.insert(msg.user_id, msg.connection) {
            old_connection.do_send(Close {
                reason: "Only one connection per user".to_owned(),
            });
        }
    }
}
