use crate::messages::Message;
use crate::player::Player;

pub struct GameEnvironment {
    pub player: Player,
    pub opponent: Player,
}

impl Default for GameEnvironment {
    fn default() -> Self {
        GameEnvironment {
            player: Player::default(),
            opponent: Player::default(),
        }
    }
}

impl GameEnvironment {
    pub fn game_loop(&mut self, msg: &Message) {
        match msg {
            Message::ChangeActive(t) => {
                self.player.active_character = *t;
            }
            Message::UseNormalAttack => {}
            Message::UseESkill => {}
            Message::UseQSkill => {}
            Message::UseActionCard(t) => {}
        }
    }
}
