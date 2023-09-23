use actix::Message;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Close {
    pub reason: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct SendClientMessage {
    pub message: String,
}
