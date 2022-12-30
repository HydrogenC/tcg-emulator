use crate::dice_set::DiceSet;
use crate::player::Player;

pub struct GameEnvironment{
    pub player: Player,
    pub opponent: Player
}

impl Default for GameEnvironment {
    fn default() -> Self {
        GameEnvironment{
            player: Player::default(),
            opponent: Player::default()
        }
    }
}
