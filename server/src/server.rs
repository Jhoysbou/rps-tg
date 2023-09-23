use actix::{Actor, Context};

pub struct Server {}

impl Actor for Server {
    type Context = Context<Self>;
}
