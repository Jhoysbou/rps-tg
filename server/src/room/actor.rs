use actix::{Actor, Addr, Context};
use uuid::Uuid;

use crate::{server::actor::Server, types::UserId};

pub struct Room {
    id: Uuid,
    server: Addr<Server>,
    first_user: UserId,
    second_user: UserId,
    rounds: Vec<Round>,
    rounds_count: u8,
}

struct Round {
    status: RoundStatus,
    first_player_action: Option<Action>,
    second_player_action: Option<Action>,
    winner: Option<UserId>,
}

enum RoundStatus {
    InProgress,
    Completed,
}

enum Action {
    Rock,
    Paper,
    Scissors,
}

impl Actor for Room {
    type Context = Context<Self>;
}

impl Room {
    pub fn new(id: Uuid, server: Addr<Server>, first_user: UserId, second_user: UserId) -> Self {
        Self {
            id,
            server,
            first_user,
            second_user,
            rounds_count: 0,
            rounds: vec![Round {
                status: RoundStatus::InProgress,
                first_player_action: None,
                second_player_action: None,
                winner: None,
            }],
        }
    }
}
