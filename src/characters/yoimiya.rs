use std::sync::Arc;
use crate::operation_context::OperationContext;
use crate::characters::character::{Character, CharacterHandler};
use crate::dice_set::ElementType;
use crate::game_environment::GameEnvironment;

struct YoimiyaHandler {
    pyro_attached: bool,
}

impl Default for YoimiyaHandler {
    fn default() -> Self {
        YoimiyaHandler {
            pyro_attached: false
        }
    }
}

impl CharacterHandler for YoimiyaHandler {
    fn on_normal_attack(&mut self, info: OperationContext, env: &mut GameEnvironment) {
        let dmg = if self.pyro_attached { 4 } else { 2 };
        env.players[info.target_player].characters[info.target_character].hp -= dmg;
    }

    fn on_e_skill(&mut self, info: OperationContext, env: &mut GameEnvironment) {
        self.pyro_attached = true;
    }

    fn on_q_skill(&mut self, info: OperationContext, env: &mut GameEnvironment) {
        todo!()
    }
}

pub fn yoimiya() -> Character {
    Character {
        name: "Yoimiya",
        max_hp: 10,
        hp: 10,
        e_cost: 1,
        q_cost: 3,
        element: ElementType::Pyro,
        handler: Arc::new(YoimiyaHandler::default()),
    }
}