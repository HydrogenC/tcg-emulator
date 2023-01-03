use std::sync::Arc;
use crate::characters::character::CharacterHandler;
use crate::messages::{Message, SkillType};
use crate::player::Player;

pub struct GameEnvironment {
    pub player: Player,
    pub opponent: Player,
    pub reroll_chances: usize,
}

impl Default for GameEnvironment {
    fn default() -> Self {
        GameEnvironment {
            player: Player::default(),
            opponent: Player::default(),
            reroll_chances: 0,
        }
    }
}

impl GameEnvironment {
    pub fn game_loop(&mut self, msg: &Message) {
        match msg {
            Message::ChangeActive(t) => {
                self.player.active_character = *t;
            }
            Message::UseActionCard(s, t) => {
                s.use_card(*t, self);
            }
            Message::TurnEnd => {
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
            }
            Message::TurnStart => {
                self.reroll_chances = 1;
            }
            Message::UseSkill(s, t) => {
                let raw_handler =
                    Arc::into_raw(self.player.characters[self.player.active_character].handler.clone())
                        as *mut dyn CharacterHandler;
                // This is safe because handler will only be read in one thread
                unsafe {
                    match s {
                        SkillType::NormalAttack => {
                            (*raw_handler).on_normal_attack(self.player.active_character, *t, self);
                        }
                        SkillType::ESkill => {
                            (*raw_handler).on_e_skill(self.player.active_character, *t, self);
                        }
                        SkillType::QSkill => {
                            (*raw_handler).on_q_skill(self.player.active_character, *t, self);
                        }
                    }
                }
            }
        }
    }
}
