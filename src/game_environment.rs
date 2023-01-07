use std::sync::Arc;
use std::sync::mpsc::Sender;
use actix::Addr;
use crate::characters::character::CharacterHandler;
use crate::dice_set::ElementType;
use crate::game_events::{GameEvents, SkillType};
use crate::GameActor;
use crate::player::Player;
use crate::server_messages::FetchCharacterListMessage;

pub struct GameEnvironment {
    pub player: Player,
    pub opponent: Player,
    pub game_actor_addr: Addr<GameActor>,
}

impl GameEnvironment {
    pub(crate) fn new(addr: Addr<GameActor>) -> Self {
        GameEnvironment {
            player: Player::default(),
            opponent: Player::default(),
            game_actor_addr: addr,
        }
    }
}

impl GameEnvironment {
    pub fn game_loop(&mut self, msg: &GameEvents, send: &Sender<GameEvents>) {
        match msg {
            GameEvents::RequestCharacterList => {
                self.game_actor_addr.do_send(FetchCharacterListMessage {
                    player_characters: self.player.characters.iter().map(|a| {
                        a.name.to_string()
                    }).collect(),
                    opponent_characters: self.opponent.characters.iter().map(|a| {
                        a.name.to_string()
                    }).collect(),
                });
            }

            GameEvents::ChangeActive(t) => {
                self.player.active_character = *t;
            }

            GameEvents::UseActionCard(card, target) => {
                // self.player.use_card(*target, self);
            }

            GameEvents::TurnEnd => {
                let player_summoned_area = self.player.summoned_area.clone();
                for i in 0..player_summoned_area.len() {
                    player_summoned_area[i].on_turn_end(self);

                    if player_summoned_area[i].remaining_uses() == 0 {
                        self.player.remove_summoned(i);
                    }
                }
                drop(player_summoned_area);

                let opponent_summoned_area = self.opponent.summoned_area.clone();
                for i in 0..opponent_summoned_area.len() {
                    opponent_summoned_area[i].on_turn_end(self);

                    if opponent_summoned_area[i].remaining_uses() == 0 {
                        self.opponent.remove_summoned(i);
                    }
                }
                drop(opponent_summoned_area);

                let player_support_area = self.player.support_area.clone();
                for i in player_support_area.iter() {
                    i.on_turn_end(self);
                }
                drop(player_support_area);

                let opponent_support_area = self.opponent.support_area.clone();
                for i in opponent_support_area.iter() {
                    i.on_turn_end(self);
                }
                drop(opponent_support_area);

                send.send(GameEvents::TurnStart).expect("TODO: panic message");
            }

            GameEvents::TurnStart => {
                self.player.reroll_chances = 1;

                self.player.dice_set.roll_dices();
                let player_elements = self.player.get_character_elements();
                self.player.dice_set.sort_dice(player_elements);

                let player_support_area = self.player.support_area.clone();
                for i in player_support_area.iter() {
                    i.on_turn_start(self);
                }
                drop(player_support_area);

                let opponent_support_area = self.opponent.support_area.clone();
                for i in opponent_support_area.iter() {
                    i.on_turn_start(self);
                }
                drop(opponent_support_area);
            }

            GameEvents::UseSkill(skill, target, cost) => {
                let raw_handler =
                    Arc::into_raw(self.player.characters[self.player.active_character].handler.clone())
                        as *mut dyn CharacterHandler;
                // This is safe because handler will only be read in one thread
                unsafe {
                    match skill {
                        SkillType::NormalAttack => {
                            (*raw_handler).on_normal_attack(self.player.active_character, *target, self);
                        }
                        SkillType::ESkill => {
                            (*raw_handler).on_e_skill(self.player.active_character, *target, self);
                        }
                        SkillType::QSkill => {
                            (*raw_handler).on_q_skill(self.player.active_character, *target, self);
                        }
                    }
                }

                for i in cost.iter() {
                    self.player.dice_set.dices[*i] = ElementType::Null;
                }

                let player_elements = self.player.get_character_elements();
                self.player.dice_set.sort_dice(player_elements);
                self.player.dice_set.dice_count -= cost.len();
            }

            GameEvents::RerollDice(dices) => {
                if dices.len() == 0 {
                    self.player.reroll_chances = 0;
                } else {
                    for i in dices.iter() {
                        self.player.dice_set.reroll_dice(*i);
                    }

                    let player_elements = self.player.get_character_elements();
                    self.player.dice_set.sort_dice(player_elements);
                    self.player.reroll_chances -= 1;
                }
            }

            _ => {}
        }
    }
}
