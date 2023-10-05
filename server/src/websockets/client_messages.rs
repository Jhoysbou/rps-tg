use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    room::actor::Action,
    server::messages::{MatchmakingStatus, ProcessClientMessageResult},
    types::UserId,
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "data")]
pub enum IncomingClientMessage {
    StartMatchmaking,
    MakeAction(MakeActionPayload),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "data")]
pub enum OutgoingClientMessage {
    Error(ErrorPayload),
    ConfirmConnect(ConfirmConnectPayload),
    MatchmakingSuccess(MatchmakingSuccessPayload),
    MatchmakingStarted,
    MakeActionSuccess,
}

impl From<ProcessClientMessageResult> for OutgoingClientMessage {
    fn from(value: ProcessClientMessageResult) -> Self {
        match value {
            ProcessClientMessageResult::StartMatchmakingResult(payload) => match payload.status {
                MatchmakingStatus::Searching => OutgoingClientMessage::MatchmakingStarted,
                MatchmakingStatus::Found => {
                    OutgoingClientMessage::MatchmakingSuccess(MatchmakingSuccessPayload {
                        opponent: payload.opponent.unwrap(),
                        room: payload.room.unwrap(),
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
pub struct MakeActionPayload {
    pub room: Uuid,
    pub action: Action,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MatchmakingSuccessPayload {
    pub room: Uuid,
    pub opponent: UserId,
}
