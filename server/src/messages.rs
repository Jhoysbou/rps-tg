use actix::{Addr, Message};

use crate::{types::UserId, websockets::Connection};

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub connection: Addr<Connection>,
    pub user_id: UserId,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Close {
    pub reason: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct SendWSMessage {
    pub message: String,
}
