use std::collections::HashMap;

use actix::{Actor, Addr, Context, Handler};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{server::actor::Server, types::UserId};

use super::{
    error::RoomError,
    messages::{GameFinishedResult, MakeAction, MakeActionResult, RoundFinishedResult},
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
    fn add_action(&mut self, action: UserAction) {
        self.actions.push(action);
    }

    fn decide_winner(&mut self) -> Option<UserId> {
        let first_user = self.actions.first()?;
        let second_user = self.actions.last()?;

        let winner;

        if (first_user.action as u8 + 1u8) % 3 == second_user.action as u8 {
            winner = Some(second_user.user_id);
        } else if first_user.action as u8 == second_user.action as u8 {
            winner = None;
        } else {
            winner = Some(first_user.user_id);
        }

        self.winner = winner;
        self.winner
    }
}

#[derive(Clone, Copy)]
pub struct UserAction {
    user_id: UserId,
    action: Action,
}

enum RoundStatus {
    InProgress,
    Completed,
}

#[derive(Serialize, Deserialize, Debug, Display, Clone, Copy)]
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
            rounds: vec![Room::new_round()],
        }
    }

    fn new_round() -> Round {
        Round {
            status: RoundStatus::InProgress,
            actions: vec![],
            winner: None,
        }
    }

    fn is_game_over(&self) -> (bool, Option<UserId>) {
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
            .filter(|(_user_id, wins)| **wins >= WINS_REQURED)
            .map(|(user_id, _)| *user_id)
            .collect::<Vec<UserId>>();

        if winner.len() == 0 {
            return (false, None);
        } else if winner.len() > 1 {
            // Drow
            return (true, None);
        }

        (true, winner.first().cloned())
    }
}

impl Handler<MakeAction> for Room {
    type Result = Result<MakeActionResult, RoomError>;

    fn handle(&mut self, msg: MakeAction, _ctx: &mut Self::Context) -> Self::Result {
        let round = self.rounds.last_mut().ok_or(RoomError {
            message: "Room initialization error. Try again".to_owned(),
        })?;

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

        round.add_action(UserAction {
            user_id: msg.user_id,
            action: msg.action,
        });

        let actions = round.actions.clone();

        if round.actions.len() == 2 {
            let winner = round.decide_winner();
            self.rounds_count += 1;

            let (is_finished, game_winner) = self.is_game_over();

            if is_finished {
                return Ok(MakeActionResult::GameFinished(GameFinishedResult {
                    actions,
                    winner: game_winner,
                }));
            }

            return Ok(MakeActionResult::RoundFinished(RoundFinishedResult {
                winner,
                actions,
                next_round_cound: self.rounds_count,
            }));
        }

        Ok(MakeActionResult::Accepted)
    }
}
