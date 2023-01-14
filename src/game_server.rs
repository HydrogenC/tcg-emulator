use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::sync::mpsc::{channel, Sender};
use std::thread;
use actix::{Actor, Addr, Context, Handler, Message};
use crate::game_environment::GameEnvironment;
use crate::game_events::GameEvent;
use crate::player_session::PlayerSession;

pub struct EnterRoomMessage {
    pub addr: Addr<PlayerSession>,
    pub room_id: usize,
}

pub struct EnterRoomResult {
    pub player_index: usize,
    pub sender: Sender<GameEvent>,
}

impl Message for EnterRoomMessage {
    type Result = Result<EnterRoomResult, std::io::Error>;
}

#[derive(Clone)]
pub struct GameInstance {
    env: Arc<RwLock<GameEnvironment>>,
    send: Sender<GameEvent>,
}

pub struct GameServer {
    games: HashMap<usize, GameInstance>,
}

impl GameServer {
    pub fn new() -> Self {
        GameServer {
            games: HashMap::new()
        }
    }
}

impl Handler<EnterRoomMessage> for GameServer {
    type Result = Result<EnterRoomResult, std::io::Error>;

    fn handle(&mut self, msg: EnterRoomMessage, ctx: &mut Self::Context) -> Self::Result {
        if !self.games.contains_key(&msg.room_id) {
            let game_env = Arc::new(RwLock::new(GameEnvironment::new()));
            let (send, recv) = channel();

            // Clone the variables for the second thread
            let game_env_clone = game_env.clone();
            let game_ended_clone = { game_env.read().unwrap().game_ended.clone() };
            let send_clone = send.clone();
            thread::spawn(move || {
                loop {
                    let msg = recv.recv().unwrap();
                    let mut env = game_env_clone.write().unwrap();

                    if *game_ended_clone.read().unwrap() {
                        break;
                    }

                    println!("Got client message");
                    env.handle_message(&msg, &send_clone);
                }
            });

            self.games.insert(msg.room_id, GameInstance {
                env: game_env,
                send,
            });
        }
        let game_arc = self.games.get(&msg.room_id).unwrap();

        let mut game_env = game_arc.env.write().unwrap();
        let player_index = game_env.add_player(msg.addr);

        Ok(EnterRoomResult {
            player_index,
            sender: game_arc.send.clone(),
        })
    }
}

impl Actor for GameServer {
    type Context = Context<Self>;
}