# Backend documentation
## Build and run 
You need to have rust and cargo installed.
[How to install rust and cargo?](https://www.rust-lang.org/tools/install)

```bash
cd server
```
Run the backend
```bash
cargo run
```
This command will download, compile all the dependencies alongside the source code and run it.

The server will be accessible on port `8080`. It has one websocket endpoint `/ws/{userId}`
All communication comes through the websocket connection.
## Structure
```
├── Cargo.lock
├── Cargo.toml
└── src
    ├── main.rs
    ├── room
    │   ├── actor.rs
    │   ├── error.rs
    │   └── messages.rs
    ├── room.rs
    ├── server
    │   ├── actor.rs
    │   ├── error.rs
    │   └── messages.rs
    ├── server.rs
    ├── types.rs
    ├── websockets
    │   ├── client_messages.rs
    │   ├── messages.rs
    │   └── ws.rs
    └── websockets.rs
5 directories, 16 files
```

## Actors
> Actix is built on the Actor Model which allows applications to be written as a group of independently executing but cooperating "Actors" which communicate via messages. Actors are objects which encapsulate state and behavior and run within the Actor System provided by the actix library.

[About actors](https://actix.rs/docs/actix/actor)

The server consists of three actors:
- `Server` — ./src/server/actor.rs
- `Connection` — ./src/websockets/ws.rs
- `Room` — ./src/room/actor.rs

Actors are communicating through messages. For each actor there is a file called `messages.rs` that contains the messages a particular actor can handle. They are pretty simple so I won't explain them here.

### Server
Server actor is a singleton object created at the application startup.
```rust
pub struct Server {
    connections: HashMap<UserId, Addr<Connection>>,
    matchmaking_queue: VecDeque<UserId>,
    rooms: HashMap<RoomId, Addr<Room>>,
}
```
The main job for a server is to manage connections and rooms.

### Connection
Connection actor is created each time a new websocket connection is established.
```rust
pub struct Connection {
    user_id: UserId,
    server: Addr<Server>,
    last_ping: Instant,
}
```
It holds a websocket connection and can recieve and send messages.

### Room
Room actor represents a game room. Created each time the game between two people is started.
```rust
pub struct Room {
    id: Uuid,
    server: Addr<Server>,
    users: [UserId; 2],
    rounds: Vec<Round>,
    rounds_count: u8,
}
```
The room actor is aware of the game rules. So the job of the room actor is to apply those rules and store a state of a particular game.


## Websocket messages
The server and client communicate through a set of messages.
These messages are listed here [client_messages.rs](/src/websockets/client_messages.rs)
All messages are serialized to json with such structure:
```json
{
    "type": "<MESSAGE_TYPE>",
    "data": {<OPTIONAL_MESSAGE_PAYLOAD>}
}
```

### Incoming messages
There are only two types of incoming messages:
- `StartMatchmaking`
- `MakeAction`

Incoming message are just a rust enum.
```rust
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "data")]
pub enum IncomingClientMessage {
    StartMatchmaking,
    MakeAction(MakeActionPayload),
}
```
These macros are needed to properly serialize the enum to a json representation.
```rust
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "data")]
```
#### StartMatchmaking
The message has no payload and just puts a user to the matchmaking queue.
#### MakeAction
The message is sent when a player has chosen an action (Rock, Paper or Scissors).

Payload contains of a room id and a string representing one of the actions.
```rust
pub struct MakeActionPayload {
    pub room: Uuid,
    pub action: Action,
}
```

### Outgoing messages
There are seven outgoing messages
They are listed in the same file as the incoming mesage.

Rust enum with message.
```rust
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
```
I'm not going to explain every one of them because they work the same way as the incoming messages.
The detailed structure of this message you can view [here](/src/websockets/client_messages.rs)

## Code walkthrough
This section contains explanation of the key parts of the server logic.

### Connection
Start Actix Web server and create the server actor.

[`main.rs`]
```rust
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();
    log::info!("Starting server...");

    let server = Server::new().start();

    HttpServer::new(move ||
        App::new()
            .service(start_connection)
            .app_data(Data::new(server.clone()))
        )
        .bind(("::", 8080))?
        .run()
        .await
}
```

When a websocket connection reaches the server `start_connection` function is called.
```rust
#[get("/ws/{user_id}")]
pub async fn start_connection(
    req: HttpRequest,
    stream: Payload,
    user_id: Path<UserId>,
    srv: Data<Addr<Server>>,
) -> Result<HttpResponse, Error> {
    let conn = Connection::new(
        *user_id,
        srv.get_ref().clone(),
    );

    let resp = ws::start(conn, &req, stream)?;
    Ok(resp)
}
```
We create a new connection actor and proceed to websocket handshake. Luckily, it is implemented in Actix Web.
[More about websockets](https://developer.mozilla.org/en-US/docs/Web/API/WebSockets_API/Writing_WebSocket_servers)

When a message is sent to the server. The `StreamHandler` is called. StreamHandler processes the messages.
The handler is implemented for our actor `Connection`

[`websockets/ws.rs`]
```rust
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Connection {
```

There are several messages types Actix provides us an interface to handle. We focus here on `ws::Message::Text()`

```rust
ws::Message::Text(msg) => match serde_json::from_str::<IncomingClientMessage>(&msg) {
    Ok(message) => {
        log::debug!("Event from user {} with type {:#?}", self.user_id, message);
        self.server
            .send(ProcessClientMessage {
                message,
                user_id: self.user_id,
            })
            .into_actor(self)
            .then(|res, conn, ctx| {
                if let Err(err) = res {
                    log::error!("Couldn't send message to actor: {}", err);
                    return fut::ready(());
                }

                match res.unwrap() {
                    Ok(result) => {
                        conn.send_message(OutgoingClientMessage::from(result), ctx);
                    }
                    Err(err) => {
                        log::error!("Process message error: {}", err);
                        conn.send_message(
                            OutgoingClientMessage::Error(ErrorPayload {
                                message: "Internal error: Couldn't process event"
                                    .to_owned(),
                            }),
                            ctx,
                        );
                    }
                }

                fut::ready(())
            })
            .wait(ctx);
    }
    Err(err) => {
        log::warn!("Couldn't parse message: {} because of {}", msg, err);
        self.send_message(
            OutgoingClientMessage::Error(ErrorPayload {
                message: "Bad request".to_owned(),
            }),
            ctx,
        )
    }
},
```

First, we parse the text message to the struct
```rust
match serde_json::from_str::<IncomingClientMessage>(&msg)
```
On success, we send the parsed message to the `Server` actor.

```rust
self.server
    .send(ProcessClientMessage {
        message,
        user_id: self.user_id,
    })
    .into_actor(self)
```
We wrap the client message, adding user_id field.

The server will return something back for us, so we need to listen:
```rust
.then(|res, conn, ctx| {
    if let Err(err) = res {
        log::error!("Couldn't send message to actor: {}", err);
        return fut::ready(());
    }

    match res.unwrap() {
        Ok(result) => {
            conn.send_message(OutgoingClientMessage::from(result), ctx);
        }
        Err(err) => {
            log::error!("Process message error: {}", err);
            conn.send_message(
                OutgoingClientMessage::Error(ErrorPayload {
                    message: "Internal error: Couldn't process event"
                        .to_owned(),
                }),
                ctx,
            );
        }
    }

    fut::ready(())
})
```
We handle errors and send result to the client.

So `Connection` actor is pretty simple, it just passes messages back and forth.

### Server
Okay, let's see what Server actor is actually doing

[`server/actor.rs`]
```rust
impl Handler<ProcessClientMessage> for Server {
    type Result = ResponseActFuture<Self, Result<ProcessClientMessageResult, ServerError>>;

    fn handle(&mut self, msg: ProcessClientMessage, ctx: &mut Self::Context) -> Self::Result {
        match msg.message {
            IncomingClientMessage::StartMatchmaking => {
                ...
            }
            IncomingClientMessage::MakeAction(payload) => {
                ...
            }
        }
    }
}
```

The server is processing incoming messages from client.

Let's look deeply for these messages:

#### StartMatchmaking
```rust
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
```
First, we check if there is someone waiting in the queue.
If it is so, we remove the connection from queue and create a new room for a game
```rust
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
```

Then we just send a message to the opponent from the queue and return a similar message to the Connection. 

```rust
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
```


If there is no one in the queue, we simply add a connection to the queue and return result message
```rust
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
```

#### MakeAction
```rust
if let Some(room) = self.rooms.get(&payload.room) {
    Box::pin(
        room.send(MakeAction {
            action: payload.action,
            user_id: msg.user_id,
        })
        .into_actor(self)
        .then(move |res, server, _ctx| {
            if let Err(err) = res {
                log::error!("Couldn't send message to room: {}", err);
                return fut::ready(Err(ServerError {
                    message: "Internal error, try again".to_owned(),
                }));
            }

            let res = match res.unwrap() {
                Ok(ref make_action_res) => {
                    match make_action_res {
                        MakeActionResult::RoundFinished(result) => {
                            let opponent = result
                                .users
                                .iter()
                                .find(|u| **u != msg.user_id)
                                .unwrap();
                            let opponent_connection =
                                server.connections.get(&opponent).unwrap();

                            opponent_connection.do_send(SendClientMessage {
                                message: OutgoingClientMessage::from(
                                    ProcessClientMessageResult::MakeActionResult(
                                        make_action_res.clone(),
                                    ),
                                ),
                            })
                        }
                        MakeActionResult::GameFinished(result) => {
                            let opponent = result
                                .users
                                .iter()
                                .find(|u| **u != msg.user_id)
                                .unwrap();
                            let opponent_connection =
                                server.connections.get(&opponent).unwrap();

                            opponent_connection.do_send(SendClientMessage {
                                message: OutgoingClientMessage::from(
                                    ProcessClientMessageResult::MakeActionResult(
                                        make_action_res.clone(),
                                    ),
                                ),
                            })
                        }
                        _ => (),
                    };
                    Ok(ProcessClientMessageResult::MakeActionResult(
                        make_action_res.clone(),
                    ))
                }
                Err(err) => Err(ServerError::from(err)),
            };

            fut::ready(res)
        }),
    )
```

Find a room actor and send these message to the actor

```rust
    Box::pin(
        room.send(MakeAction {
            action: payload.action,
            user_id: msg.user_id,
        })
        .into_actor(self)
```

Result of this call we return to the Connection actor:
```rust
.then(move |res, server, _ctx| {
    if let Err(err) = res {
        log::error!("Couldn't send message to room: {}", err);
        return fut::ready(Err(ServerError {
            message: "Internal error, try again".to_owned(),
        }));
    }

    let res = match res.unwrap() {
        Ok(ref make_action_res) => {
            match make_action_res {
                MakeActionResult::RoundFinished(result) => {
                    let opponent = result
                        .users
                        .iter()
                        .find(|u| **u != msg.user_id)
                        .unwrap();
                    let opponent_connection =
                        server.connections.get(&opponent).unwrap();

                    opponent_connection.do_send(SendClientMessage {
                        message: OutgoingClientMessage::from(
                            ProcessClientMessageResult::MakeActionResult(
                                make_action_res.clone(),
                            ),
                        ),
                    })
                }
                MakeActionResult::GameFinished(result) => {
                    let opponent = result
                        .users
                        .iter()
                        .find(|u| **u != msg.user_id)
                        .unwrap();
                    let opponent_connection =
                        server.connections.get(&opponent).unwrap();

                    opponent_connection.do_send(SendClientMessage {
                        message: OutgoingClientMessage::from(
                            ProcessClientMessageResult::MakeActionResult(
                                make_action_res.clone(),
                            ),
                        ),
                    })
                }
                _ => (),
            };
            Ok(ProcessClientMessageResult::MakeActionResult(
                make_action_res.clone(),
            ))
        }
        Err(err) => Err(ServerError::from(err)),
    };

    fut::ready(res)
}),
```

This long scary code is just handles error and prepares messages for the Connection actor and, of course, sends the same message to the opponent.


### Room
Room actor handles only one message `MakeAction`:

```rust
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
        round.finish();

        let (is_finished, game_winner) = self.is_game_over();

        if is_finished {
            return Ok(MakeActionResult::GameFinished(GameFinishedResult {
                actions,
                winner: game_winner,
                users: self.users,
            }));
        }

        self.start_new_round();

        return Ok(MakeActionResult::RoundFinished(RoundFinishedResult {
            winner,
            actions,
            next_round_cound: self.rounds_count,
            users: self.users,
        }));
    }

    Ok(MakeActionResult::Accepted)
}
```

After handling some errors, we add the player action to the `Round` struct
```rust
round.add_action(UserAction {
    user_id: msg.user_id,
    action: msg.action,
});

let actions = round.actions.clone();
```

And then we need to determine if a current game round is finished or maybe the whole game is finished with this player's action:

```rust
if round.actions.len() == 2 {
    let winner = round.decide_winner();
    self.rounds_count += 1;
    round.finish();

    let (is_finished, game_winner) = self.is_game_over();

    if is_finished {
        return Ok(MakeActionResult::GameFinished(GameFinishedResult {
            actions,
            winner: game_winner,
            users: self.users,
        }));
    }

    self.start_new_round();

    return Ok(MakeActionResult::RoundFinished(RoundFinishedResult {
        winner,
        actions,
        next_round_cound: self.rounds_count,
        users: self.users,
    }));
}
```

The round is finished if both players made actions. We finish this round and check if the game is finished.
```rust
if round.actions.len() == 2 {
    let winner = round.decide_winner();
    self.rounds_count += 1;
    round.finish();

    let (is_finished, game_winner) = self.is_game_over();

    if is_finished {
        return Ok(MakeActionResult::GameFinished(GameFinishedResult {
            actions,
            winner: game_winner,
            users: self.users,
        }));
    }
    ...
```

Let's see the logic for the `round.decide_winner()` and `self.is_game_over()`
```rust
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
```

We represent an action (rock, paper, scissors) as integers (0, 1, 2 respectively)
By taking the remainder of the division we can determine if the opponent player has chosen a winner action. We return the winner on None if they chose the same action.


Now for `is_game_over()`
```rust
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
```
We count wins of each player by every round
and if one them is scored more than 2 (`WINS_REQURED`) we suppose the game is over and return the winner ID.
