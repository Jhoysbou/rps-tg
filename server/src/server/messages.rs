use actix::{Addr, Message};

use crate::{transport::ws::Connection, types::UserId};

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub connection: Addr<Connection>,
    pub user_id: UserId,
}
