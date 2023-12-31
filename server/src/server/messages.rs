use actix::{Addr, Message};
use uuid::Uuid;

use crate::{
    room::messages::MakeActionResult,
    server::error::ServerError,
    types::UserId,
    websockets::{client_messages::IncomingClientMessage, ws::Connection},
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
    MakeActionResult(MakeActionResult),
}

pub enum MatchmakingStatus {
    Searching,
    Found,
}

pub struct StartMatchmakingResultPayload {
    pub opponent: Option<UserId>,
    pub room: Option<Uuid>,
    pub status: MatchmakingStatus,
}
