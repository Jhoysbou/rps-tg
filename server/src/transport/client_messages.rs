use serde::{Deserialize, Serialize};

use crate::{
    server::messages::{MatchmakingStatus, ProcessClientMessageResult},
    types::UserId,
};

#[derive(Serialize, Deserialize, Debug)]
pub enum IncomingClientMessage {
    StartMatchmaking(StartMatchmakingPayload),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum OutgoingClientMessage {
    Error(ErrorPayload),
    ConfirmConnect(ConfirmConnectPayload),
    MatchmakingSuccess(MatchmakingSuccessPayload),
    MatchmakingStarted,
}

impl From<ProcessClientMessageResult> for OutgoingClientMessage {
    fn from(value: ProcessClientMessageResult) -> Self {
        match value {
            ProcessClientMessageResult::StartMatchmakingResult(payload) => match payload.status {
                MatchmakingStatus::Searching => OutgoingClientMessage::MatchmakingStarted,
                MatchmakingStatus::Found => {
                    OutgoingClientMessage::MatchmakingSuccess(MatchmakingSuccessPayload {
                        opponent: payload.opponent.unwrap(),
                    })
                }
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorPayload {
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfirmConnectPayload {
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StartMatchmakingPayload {
    pub user_id: UserId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MatchmakingSuccessPayload {
    pub opponent: UserId,
}
