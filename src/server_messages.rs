use actix::prelude::*;
use serde::{Deserialize, Serialize};

// Messages sent to the client

#[derive(Serialize)]
pub struct PlayerState {
    hp: usize,
    support_area: Vec<(String, usize)>,
    summoned_area: Vec<(String, usize)>,
    active_character: usize,
    dice_set: Vec<usize>,
}

#[derive(Serialize, Message)]
#[rtype(result = "()")]
pub struct UpdateStateMessage {
    pub player_state: PlayerState,
    pub opponent_state: PlayerState,
    pub players_turn: bool,
}

#[derive(Serialize, Message)]
#[rtype(result = "()")]
pub struct SetupClientMessage {
    pub player_index: usize,
    pub player_characters: Vec<String>,
    pub opponent_characters: Vec<String>,
}

#[derive(Serialize, Message)]
#[rtype(result = "()")]
pub struct UpdateDicesMessage {
    pub dice_set: Vec<i8>,
}

#[derive(Serialize, Message)]
#[rtype(result = "()")]
pub struct TurnOfMessage{
    pub turn_of: usize
}