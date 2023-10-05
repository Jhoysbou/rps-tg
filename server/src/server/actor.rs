use std::collections::{HashMap, VecDeque};

use actix::*;
use uuid::Uuid;

use crate::{
    room::{actor::Room, messages::MakeAction},
    types::{RoomId, UserId},
    websockets::{
        client_messages::{
            IncomingClientMessage, MatchmakingSuccessPayload, OutgoingClientMessage,
        },
        messages::{Close, SendClientMessage},
        ws::Connection,
    },
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
    rooms: HashMap<RoomId, Addr<Room>>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            connections: HashMap::new(),
            matchmaking_queue: VecDeque::new(),
            rooms: HashMap::new(),
        }
    }
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
    type Result = ResponseActFuture<Self, Result<ProcessClientMessageResult, ServerError>>;

    fn handle(&mut self, msg: ProcessClientMessage, ctx: &mut Self::Context) -> Self::Result {
        match msg.message {
            IncomingClientMessage::StartMatchmaking => {
                if self.matchmaking_queue.len() > 0 {
                    let opponent = self.matchmaking_queue.pop_front().unwrap();
                    let opponent_connection = self.connections.get(&opponent);

                    if opponent_connection.is_none() {
                        log::error!("Connection not found!");
                        return Box::pin(fut::ready(Err(ServerError {
                            message: "Opponent connection is not initialized".to_owned(),
                        })));
                    }

                    let room_id = Uuid::new_v4();
                    let room = Room::new(room_id, ctx.address(), msg.user_id, opponent).start();
                    self.rooms.insert(room_id, room);

                    // Send message to the opponent about success matchmaking
                    opponent_connection.unwrap().do_send(SendClientMessage {
                        message: OutgoingClientMessage::MatchmakingSuccess(
                            MatchmakingSuccessPayload {
                                room: room_id,
                                opponent: msg.user_id,
                            },
                        ),
                    });

                    Box::pin(fut::ready(Ok(
                        ProcessClientMessageResult::StartMatchmakingResult(
                            StartMatchmakingResultPayload {
                                opponent: Some(opponent),
                                status: MatchmakingStatus::Found,
                                room: Some(room_id),
                            },
                        ),
                    )))
                } else {
                    self.matchmaking_queue.push_back(msg.user_id);

                    Box::pin(fut::ready(Ok(
                        ProcessClientMessageResult::StartMatchmakingResult(
                            StartMatchmakingResultPayload {
                                opponent: None,
                                status: MatchmakingStatus::Searching,
                                room: None,
                            },
                        ),
                    )))
                }
            }
            IncomingClientMessage::MakeAction(payload) => {
                if let Some(room) = self.rooms.get(&payload.room) {
                    Box::pin(
                        room.send(MakeAction {
                            action: payload.action,
                            user_id: msg.user_id,
                        })
                        .into_actor(self)
                        .then(|res, _room, _ctx| {
                            if let Err(err) = res {
                                log::error!("Couldn't send message to room: {}", err);
                                return fut::ready(Err(ServerError {
                                    message: "Internal error, try again".to_owned(),
                                }));
                            }

                            let res = match res.unwrap() {
                                Ok(make_action_res) => Ok(
                                    ProcessClientMessageResult::MakeActionResult(make_action_res),
                                ),
                                Err(err) => Err(ServerError::from(err)),
                            };

                            fut::ready(res)
                        }),
                    )
                } else {
                    Box::pin(fut::ready(Err(ServerError {
                        message: "No such room".to_owned(),
                    })))
                }
            }
        }
    }
}
