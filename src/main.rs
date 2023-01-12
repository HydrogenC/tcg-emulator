use std::sync::{Arc, RwLock};
use std::sync::mpsc::{channel, Sender};
use std::thread;
use crate::game_environment::GameEnvironment;
use actix::{Actor, AsyncContext, Handler, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use serde_json::json;
use crate::game_events::GameEvents;
use crate::server_messages::FetchCharacterListMessage;

mod dice_set;
mod card_set;
mod cards;
mod game_environment;
mod player;
mod game_events;
mod server_messages;
mod characters;
mod operation_context;

pub struct GameActor {
    env: Option<Arc<RwLock<GameEnvironment>>>,
    game_loop_recv: Option<Sender<GameEvents>>,
    player_index: usize,
}

impl GameActor {
    fn new() -> Self {
        let ac = GameActor {
            env: None,
            game_loop_recv: None,
            player_index: 0,
        };

        return ac;
    }
}

impl Actor for GameActor {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let (send, recv) = channel();
        self.game_loop_recv = Some(send.clone());
        self.env = Some(Arc::new(RwLock::new(GameEnvironment::new(ctx.address()))));

        let game_env_cloned = self.env.as_ref().unwrap().clone();
        let player_index = self.player_index;
        thread::spawn(move || {
            loop {
                let msg = recv.recv().unwrap();
                let mut env = game_env_cloned.write().unwrap();

                if let GameEvents::TerminateGame = msg {
                    break;
                }

                env.game_loop(player_index, &msg, &send);
            }
        });
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        self.game_loop_recv.as_ref().unwrap().send(GameEvents::TerminateGame)
            .expect("TODO: panic message");
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for GameActor {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                let string = text.to_string();
                if string == "Connected" {
                    self.game_loop_recv.as_ref().unwrap()
                        .send(GameEvents::RequestCharacterList)
                        .expect("TODO: panic message");
                }
            }
            _ => (),
        }
    }
}

impl Handler<FetchCharacterListMessage> for GameActor {
    type Result = ();

    fn handle(&mut self, msg: FetchCharacterListMessage, ctx: &mut Self::Context) -> Self::Result {
        let data = serde_json::to_value(&msg).unwrap();
        let root = json!({
            "type": "CharacterList",
            "data": data
        });

        ctx.text(root.to_string());
    }
}

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(GameActor::new(), &req, stream);
    println!("{:?}", resp);
    resp
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind(("127.0.0.1", 9001))?
        .run()
        .await
}
