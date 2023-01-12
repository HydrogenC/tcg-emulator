use std::sync::Arc;
use std::sync::mpsc::Sender;
use actix::Addr;
use crate::operation_context::OperationContext;
use crate::characters::character::{Character, CharacterHandler};
use crate::dice_set::ElementType;
use crate::game_events::{GameEvents, SkillType};
use crate::GameActor;
use crate::player::Player;
use crate::server_messages::FetchCharacterListMessage;

pub struct GameEnvironment {
    pub players: [Player; 2],
    pub game_actor_addr: [Option<Addr<GameActor>>; 2],
    pub active_players: usize,
}

impl GameEnvironment {
    pub(crate) fn new(addr: Addr<GameActor>) -> Self {
        GameEnvironment {
            players: [
                Player::new(0),
                Player::new(1)
            ],
            game_actor_addr: [None, None],
            active_players: 0,
        }
    }
}

impl GameEnvironment {
    pub fn game_loop(&mut self, player_index: usize, msg: &GameEvents, send: &Sender<GameEvents>) {
        match msg {
            GameEvents::RequestCharacterList => {
                self.game_actor_addr[player_index].as_ref().unwrap().do_send(FetchCharacterListMessage {
                    player_characters: self.players[player_index].characters.iter().map(|a| {
                        a.name.to_string()
                    }).collect(),
                    opponent_characters: self.players[1usize - player_index].characters.iter().map(|a| {
                        a.name.to_string()
                    }).collect(),
                });
            }

            GameEvents::ChangeActive(t) => {
                self.players[player_index].active_character = *t;
            }

            GameEvents::UseActionCard(card, target) => {
                // self.player.use_card(*target, self);
            }

            GameEvents::TurnEnd => {
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

                send.send(GameEvents::TurnStart).expect("TODO: panic message");
            }

            GameEvents::TurnStart => {
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

            GameEvents::UseSkill(skill, cost) => {
                let raw_handler =
                    Arc::into_raw(self.players[player_index].characters[self.players[player_index].active_character].handler.clone())
                        as *mut dyn CharacterHandler;

                let context_info = OperationContext::new(
                    player_index,
                    self.players[player_index].active_character,
                    self.players[1 - player_index].active_character,
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
                    self.players[player_index].dice_set.dices[*i] = ElementType::Null;
                }

                let player_elements = self.players[player_index].get_character_elements();
                self.players[player_index].dice_set.sort_dice(player_elements);
                self.players[player_index].dice_set.dice_count -= cost.len();
            }

            GameEvents::RerollDice(dices) => {
                if dices.len() == 0 {
                    self.players[player_index].reroll_chances = 0;
                } else {
                    for i in dices.iter() {
                        self.players[player_index].dice_set.reroll_dice(*i);
                    }

                    let player_elements = self.players[player_index].get_character_elements();
                    self.players[player_index].dice_set.sort_dice(player_elements);
                    self.players[player_index].reroll_chances -= 1;
                }
            }

            _ => {}
        }
    }
}
