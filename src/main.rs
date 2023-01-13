use actix::{Actor, Addr};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use crate::game_server::GameServer;
use crate::player_session::PlayerSession;

mod dice_set;
mod card_set;
mod cards;
mod game_environment;
mod player;
mod game_events;
mod server_messages;
mod characters;
mod operation_context;
mod game_server;
mod player_session;

async fn index(req: HttpRequest, stream: web::Payload, srv: web::Data<Addr<GameServer>>) -> Result<HttpResponse, Error> {
    let resp = ws::start(
        PlayerSession::new(srv.get_ref().clone()), &req, stream);
    println!("{:?}", resp);
    resp
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = GameServer::new().start();

    HttpServer::new(move || App::new()
        .app_data(web::Data::new(server.clone()))
        .route("/", web::get().to(index)))
        .bind(("127.0.0.1", 9001))?
        .workers(2)
        .run()
        .await
}
