use std::collections::{HashMap, VecDeque};

use actix::{Actor, Addr, Context, Handler};

use crate::{
    transport::{
        client_messages::{
            IncomingClientMessage::StartMatchmaking, MatchmakingSuccessPayload,
            OutgoingClientMessage,
        },
        messages::{Close, SendClientMessage},
        ws::Connection,
    },
    types::UserId,
};

use super::{
    error::ServerError,
    messages::{
        AttachConnection, MatchmakingStatus, ProcessClientMessage, ProcessClientMessageResult,
        StartMatchmakingResultPayload,
    },
};

pub struct Server {
    connections: HashMap<UserId, Addr<Connection>>,
    matchmaking_queue: VecDeque<UserId>,
}

impl Actor for Server {
    type Context = Context<Self>;
}

impl Handler<AttachConnection> for Server {
    type Result = ();

    fn handle(&mut self, msg: AttachConnection, _ctx: &mut Self::Context) -> Self::Result {
        if let Some(old_connection) = self.connections.insert(msg.user_id, msg.connection) {
            old_connection.do_send(Close {
                reason: "Only one connection per user".to_owned(),
            });
        }
    }
}

impl Handler<ProcessClientMessage> for Server {
    type Result = Result<ProcessClientMessageResult, ServerError>;

    fn handle(&mut self, msg: ProcessClientMessage, _ctx: &mut Self::Context) -> Self::Result {
        match msg.message {
            StartMatchmaking(payload) => {
                if self.matchmaking_queue.len() > 0 {
                    let opponent = self.matchmaking_queue.pop_front().unwrap();
                    let opponent_connection = self.connections.get(&opponent);

                    if opponent_connection.is_none() {
                        log::error!("Connection not found!");
                        return Err(ServerError {
                            message: "Opponent connection is not initialized".to_owned(),
                        });
                    }

                    // Send message to the oppont about success matchmaking
                    opponent_connection.unwrap().do_send(SendClientMessage {
                        message: OutgoingClientMessage::MatchmakingSuccess(
                            MatchmakingSuccessPayload {
                                opponent: msg.user_id,
                            },
                        ),
                    });

                    Ok(ProcessClientMessageResult::StartMatchmakingResult(
                        StartMatchmakingResultPayload {
                            opponent: Some(opponent),
                            status: MatchmakingStatus::Found,
                        },
                    ))
                } else {
                    self.matchmaking_queue.push_back(payload.user_id);

                    Ok(ProcessClientMessageResult::StartMatchmakingResult(
                        StartMatchmakingResultPayload {
                            opponent: None,
                            status: MatchmakingStatus::Searching,
                        },
                    ))
                }
            }
        }
    }
}
