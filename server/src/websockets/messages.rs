use actix::Message;

use super::client_messages::OutgoingClientMessage;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Close {
    pub reason: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct SendClientMessage {
    pub message: OutgoingClientMessage,
}
