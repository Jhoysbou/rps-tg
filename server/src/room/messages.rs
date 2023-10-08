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

#[derive(Clone)]
pub enum MakeActionResult {
    Accepted,
    RoundFinished(RoundFinishedResult),
    GameFinished(GameFinishedResult),
}

#[derive(Clone)]
pub struct RoundFinishedResult {
    pub winner: Option<UserId>,
    pub actions: Vec<UserAction>,
    pub next_round_cound: u8,
    pub users: [UserId; 2],
}

#[derive(Clone)]
pub struct GameFinishedResult {
    pub winner: Option<UserId>,
    pub actions: Vec<UserAction>,
    pub users: [UserId; 2],
}
