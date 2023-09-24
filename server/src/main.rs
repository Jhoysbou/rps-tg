use actix::{Actor, Addr};
use actix_web::{web::{Payload, Data, Path}, Error, HttpRequest, HttpResponse, get, HttpServer, App};
use actix_web_actors::ws;
use server::actor::Server;
use types::UserId;

use crate::transport::ws::Connection;

mod transport;
mod server;
mod types;
mod room;

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();
    let server = Server::new().start();

    HttpServer::new(move ||
        App::new()
            .service(start_connection)
            .app_data(server.clone())
        )
        .bind(("::", 8080))?
        .run()
        .await
}
