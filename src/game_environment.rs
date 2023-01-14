use std::sync::{Arc, RwLock};
use std::sync::mpsc::Sender;
use actix::Addr;
use crate::operation_context::OperationContext;
use crate::characters::character::CharacterHandler;
use crate::dice_set::ElementType;
use crate::game_events::{GameEvent, SkillType};
use crate::player::Player;
use crate::player_session::PlayerSession;
use crate::server_messages::SetupClientMessage;

pub struct GameEnvironment {
    pub players: [Player; 2],
    pub session_addr: [Option<Addr<PlayerSession>>; 2],
    pub active_players: usize,
    pub game_ended: Arc<RwLock<bool>>,
}

impl GameEnvironment {
    pub fn new() -> Self {
        GameEnvironment {
            players: [
                Player::new(),
                Player::new()
            ],
            session_addr: [None, None],
            active_players: 0,
            game_ended: Arc::new(RwLock::new(false)),
        }
    }

    pub fn add_player(&mut self, player_addr: Addr<PlayerSession>) -> usize {
        self.session_addr[self.active_players] = Some(player_addr);
        self.active_players += 1;
        self.active_players - 1
    }

    pub fn handle_message(&mut self, msg: &GameEvent, send: &Sender<GameEvent>) {
        match msg {
            GameEvent::SetupClient(id) => {
                self.session_addr[*id].as_ref().unwrap().do_send(SetupClientMessage {
                    player_index: *id,
                    player_characters: self.players[*id].characters.iter().map(|a| {
                        a.name.to_string()
                    }).collect(),
                    opponent_characters: self.players[1usize - id].characters.iter().map(|a| {
                        a.name.to_string()
                    }).collect(),
                });
            }

            GameEvent::ChangeActive(id, t) => {
                self.players[*id].active_character = *t;
            }

            GameEvent::UseActionCard(id, card, target) => {
                // self.player.use_card(*target, self);
            }

            GameEvent::DeclareEndOfTurn(id) => {

            }

            GameEvent::TurnEnd => {
                for index in 0..2usize {
                    let summoned_area = self.players[index].summoned_area.clone();
                    for i in 0..summoned_area.len() {
                        summoned_area[i].on_turn_end(index, self);

                        if summoned_area[i].remaining_uses() == 0 {
                            self.players[index].remove_summoned(i);
                        }
                    }

                    let support_area = self.players[index].support_area.clone();
                    for i in support_area.iter() {
                        i.on_turn_end(index, self);
                    }
                }

                send.send(GameEvent::TurnStart).expect("TODO: panic message");
            }

            GameEvent::TurnStart => {
                for index in 0..2usize {
                    self.players[index].reroll_chances = 1;

                    self.players[index].dice_set.roll_dices();
                    let player_elements = self.players[index].get_character_elements();
                    self.players[index].dice_set.sort_dice(player_elements);

                    let support_area = self.players[index].support_area.clone();
                    for i in support_area.iter() {
                        i.on_turn_start(index, self);
                    }
                }
            }

            GameEvent::UseSkill(id, skill, cost) => {
                let raw_handler =
                    Arc::into_raw(self.players[*id].characters[self.players[*id].active_character].handler.clone())
                        as *mut dyn CharacterHandler;

                let context_info = OperationContext::new(
                    *id,
                    self.players[*id].active_character,
                    self.players[1 - id].active_character,
                );

                // This is safe because handler will only be read in one thread
                unsafe {
                    match skill {
                        SkillType::NormalAttack => {
                            (*raw_handler).on_normal_attack(context_info, self);
                        }
                        SkillType::ESkill => {
                            (*raw_handler).on_e_skill(context_info, self);
                        }
                        SkillType::QSkill => {
                            (*raw_handler).on_q_skill(context_info, self);
                        }
                    }
                }

                for i in cost.iter() {
                    self.players[*id].dice_set.dices[*i] = ElementType::Null;
                }

                let player_elements = self.players[*id].get_character_elements();
                self.players[*id].dice_set.sort_dice(player_elements);
                self.players[*id].dice_set.dice_count -= cost.len();
            }

            GameEvent::RerollDice(id, dices) => {
                if dices.len() == 0 {
                    self.players[*id].reroll_chances = 0;
                } else {
                    for i in dices.iter() {
                        self.players[*id].dice_set.reroll_dice(*i);
                    }

                    let player_elements = self.players[*id].get_character_elements();
                    self.players[*id].dice_set.sort_dice(player_elements);
                    self.players[*id].reroll_chances -= 1;
                }
            }
        }
    }
}
