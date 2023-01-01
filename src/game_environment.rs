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
                let player_summon_area = self.player.summon_area.clone();
                for i in player_summon_area.iter() {
                    i.on_turn_end(self);
                }
                drop(player_summon_area);

                let opponent_summon_area = self.opponent.summon_area.clone();
                for i in opponent_summon_area.iter() {
                    i.on_turn_end(self);
                }
                drop(opponent_summon_area);

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
                let handler = self.player.characters[self.player.active_character].handler.clone();
                match s {
                    SkillType::NormalAttack => {
                        handler.lock().unwrap().on_normal_attack(self.player.active_character, *t, self);
                    }
                    SkillType::ESkill => {
                        handler.lock().unwrap().on_e_skill(self.player.active_character, *t, self);
                    }
                    SkillType::QSkill => {
                        handler.lock().unwrap().on_q_skill(self.player.active_character, *t, self);
                    }
                }
            }
        }
    }
}
