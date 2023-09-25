use std::collections::HashMap;

use actix::{Actor, Addr, Context, Handler};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{server::actor::Server, types::UserId};

use super::{
    error::RoomError,
    messages::{MakeAction, MakeActionResult},
};

const WINS_REQURED: u8 = 2;

pub struct Room {
    id: Uuid,
    server: Addr<Server>,
    users: [UserId; 2],
    rounds: Vec<Round>,
    rounds_count: u8,
}

struct Round {
    status: RoundStatus,
    actions: Vec<UserAction>,
    winner: Option<UserId>,
}

impl Round {
    fn decide_winner(&mut self) -> Option<UserId> {
        let first_user = self.actions.first().unwrap();
        let second_user = self.actions.last().unwrap();

        if (first_user.action as u8 + 1u8) % 3 == second_user.action as u8 {
            return Some(second_user.user_id);
        } else if first_user.action as u8 == second_user.action as u8 {
            return None;
        } else {
            return Some(first_user.user_id);
        }
    }
}

struct UserAction {
    user_id: UserId,
    action: Action,
}

enum RoundStatus {
    InProgress,
    Completed,
}

#[derive(Serialize, Deserialize, Debug, Display)]
#[serde(untagged)]
#[repr(u8)]
pub enum Action {
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
            users: [first_user, second_user],
            rounds_count: 0,
            rounds: vec![Round {
                status: RoundStatus::InProgress,
                actions: vec![],
                winner: None,
            }],
        }
    }

    fn is_game_over(&self) -> Option<UserId> {
        let winner = self
            .rounds
            .iter()
            .fold(HashMap::<UserId, u8>::new(), |mut map, round| {
                if let Some(user_id) = round.winner {
                    *map.entry(user_id).or_default() += 1;
                }
                map
            })
            .iter()
            .filter(|(user_id, wins)| **wins >= WINS_REQURED)
            .map(|(user_id, _)| *user_id)
            .collect::<Vec<UserId>>();

        if winner.len() > 1 {
            log::error!("Found more then one winner. Something is wrong");
            return None;
        }

        winner.first().cloned()
    }
}

impl Handler<MakeAction> for Room {
    type Result = Result<MakeActionResult, RoomError>;

    fn handle(&mut self, msg: MakeAction, ctx: &mut Self::Context) -> Self::Result {
        let round = self.rounds.last();
        if round.is_none() {
            log::error!("Room initialized without first round. Use room::new()");
            return Err(RoomError {
                message: "Room initialization error. Try again".to_owned(),
            });
        }

        let round = round.unwrap();

        if !self.users.contains(&msg.user_id) {
            log::warn!(
                "User {} is trying access room {} while he is not part of it",
                msg.user_id,
                self.id,
            );
            return Err(RoomError {
                message: "You are not a part of this room".to_owned(),
            });
        }

        if round
            .actions
            .iter()
            .any(|user_action| user_action.user_id == msg.user_id)
        {
            log::warn!("User {} is trying to change its action", msg.user_id);
            return Err(RoomError {
                message: "You cannot change your action".to_owned(),
            });
        }

        round.actions.push(UserAction {
            user_id: msg.user_id,
            action: msg.action,
        });

        if round.actions.len() == 2 {
            let winner = round.decide_winner();
            self.rounds_count += 1;
        }
    }
}
