use actix::Message;

use crate::types::UserId;

use super::{
    actor::{Action, UserAction},
    error::RoomError,
};

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

pub struct RoundFinishedResult {
    pub winner: Option<UserId>,
    pub actions: Vec<UserAction>,
    pub next_round_cound: u8,
}

pub struct GameFinishedResult {
    pub winner: Option<UserId>,
    pub actions: Vec<UserAction>,
}
