use actix::Message;

use crate::types::UserId;

use super::{actor::Action, error::RoomError};

#[derive(Message)]
#[rtype(result = "Result<MakeActionResult, RoomError>")]
pub struct MakeAction {
    pub action: Action,
    pub user_id: UserId,
}

pub enum MakeActionResult {
    Accepted,
    RoundFinished(RoundFinishedResult),
    GameFinished(GameFinishedResult),
}

pub struct ActionHistory {
    user_id: UserId,
    action: Action,
}

pub struct RoundFinishedResult {
    pub winner: Option<UserId>,
    pub actions: [ActionHistory; 2],
    pub next_round_cound: u8,
}

pub struct GameFinishedResult {
    pub winner: Option<UserId>,
    pub actions: [ActionHistory; 2],
}
