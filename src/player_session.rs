use std::sync::mpsc::Sender;
use actix::{Actor, ActorContext, ActorFutureExt, ActorTryFutureExt, Addr, AsyncContext, ContextFutureSpawner, fut, Handler, StreamHandler, WrapFuture};
use actix_web_actors::ws;
use serde_json::{json, Value};
use crate::game_events::GameEvent;
use crate::game_events::GameEvent::RequestCharacterList;
use crate::game_server::{EnterRoomMessage, GameServer};
use crate::server_messages::CharacterListMessage;

#[derive(Debug)]
pub struct PlayerSession {
    server: Addr<GameServer>,
    game_loop_channel: Option<Sender<GameEvent>>,
    player_index: usize,
}

impl PlayerSession {
    pub fn new(server_addr: Addr<GameServer>) -> Self {
        let ac = PlayerSession {
            server: server_addr,
            game_loop_channel: None,
            player_index: 0,
        };

        return ac;
    }

    fn join_room(&mut self, room_id: usize, ctx: &mut <PlayerSession as Actor>::Context) {
        self.server.send(EnterRoomMessage {
            addr: ctx.address(),
            room_id,
        }).into_actor(self).then(|res, act, ctx| {
            let result = res.unwrap().unwrap();
            act.game_loop_channel = Some(result.sender);
            act.player_index = result.player_index;

            act.game_loop_channel.as_ref().unwrap().send(RequestCharacterList(act.player_index))
                .expect("TODO: panic message");
            fut::ready(())
        }).wait(ctx);
    }
}

impl Actor for PlayerSession {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for PlayerSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                let json: Value = serde_json::from_str(text.to_string().as_str()).unwrap();
                let ty = json["type"].to_string();
                match ty.as_str() {
                    "JoinRoom" => {
                        let room_id = json["room"].as_u64().unwrap();
                        self.join_room(room_id as usize, ctx);
                    }

                    _ => {}
                }
            }
            _ => (),
        }
    }
}

impl Handler<CharacterListMessage> for PlayerSession {
    type Result = ();

    fn handle(&mut self, msg: CharacterListMessage, ctx: &mut Self::Context) -> Self::Result {
        let data = serde_json::to_value(&msg).unwrap();
        let root = json!({
            "type": "CharacterList",
            "data": data
        });

        ctx.text(root.to_string());
    }
}