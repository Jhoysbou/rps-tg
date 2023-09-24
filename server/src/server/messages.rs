use actix::{Addr, Message};

use crate::{
    server::error::ServerError,
    transport::{client_messages::IncomingClientMessage, ws::Connection},
    types::UserId,
};

#[derive(Message)]
#[rtype(result = "()")]
pub struct AttachConnection {
    pub connection: Addr<Connection>,
    pub user_id: UserId,
}

#[derive(Message)]
#[rtype(result = "Result<ProcessClientMessageResult, ServerError>")]
pub struct ProcessClientMessage {
    pub message: IncomingClientMessage,
    pub user_id: UserId,
}

pub enum ProcessClientMessageResult {
    StartMatchmakingResult(StartMatchmakingResultPayload),
}

pub enum MatchmakingStatus {
    Searching,
    Found,
}

pub struct StartMatchmakingResultPayload {
    pub opponent: Option<UserId>,
    pub status: MatchmakingStatus,
}
