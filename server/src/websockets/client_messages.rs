use serde::{Deserialize, Serialize};
use std::convert::From;
use uuid::Uuid;

use crate::{
    room::{
        actor::{Action, UserAction},
        messages::MakeActionResult,
    },
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
    RoundFinished(RoundFinishedPayload),
    GameFinished(GameFinishedPayload),
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
            ProcessClientMessageResult::MakeActionResult(payload) => match payload {
                MakeActionResult::Accepted => OutgoingClientMessage::MakeActionSuccess,
                MakeActionResult::RoundFinished(round_result) => {
                    OutgoingClientMessage::RoundFinished(RoundFinishedPayload {
                        winner: round_result.winner,
                        actions: round_result
                            .actions
                            .iter()
                            .map(|user_action| ActionHistory::from(*user_action))
                            .collect(),
                        next_round_count: round_result.next_round_cound,
                    })
                }
                MakeActionResult::GameFinished(game_result) => {
                    OutgoingClientMessage::GameFinished(GameFinishedPayload {
                        winner: game_result.winner,
                        actions: game_result
                            .actions
                            .iter()
                            .map(|user_action| ActionHistory::from(*user_action))
                            .collect(),
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

#[derive(Serialize, Deserialize, Debug)]
pub struct ActionHistory {
    pub user_id: UserId,
    pub action: Action,
}

impl From<UserAction> for ActionHistory {
    fn from(value: UserAction) -> Self {
        Self {
            user_id: value.user_id,
            action: value.action,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoundFinishedPayload {
    pub winner: Option<UserId>,
    pub actions: Vec<ActionHistory>,
    pub next_round_count: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameFinishedPayload {
    pub winner: Option<UserId>,
    pub actions: Vec<ActionHistory>,
}
