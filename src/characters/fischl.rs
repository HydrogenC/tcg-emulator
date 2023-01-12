use std::sync::Arc;
use crate::operation_context::OperationContext;
use crate::cards::SummonedCard;
use crate::characters::character::{Character, CharacterHandler};
use crate::dice_set::ElementType;
use crate::game_environment::GameEnvironment;

struct FischlHandler {
    oz: Arc<Oz>,
}

struct Oz {
    lifetime: usize,
}

impl SummonedCard for Oz {
    fn on_turn_end(&self, subject_player: usize, env: &mut GameEnvironment) {
        let opponent = &mut env.players[1 - subject_player];
        opponent.characters[opponent.active_character].hp -= 1;
    }

    fn remaining_uses(&self) -> usize {
        self.lifetime
    }
}

impl Default for FischlHandler {
    fn default() -> Self {
        FischlHandler {
            oz: Arc::new(Oz { lifetime: 0 }),
        }
    }
}

impl CharacterHandler for FischlHandler {
    fn on_normal_attack(&mut self, info: OperationContext, env: &mut GameEnvironment) {
        env.players[info.target_player].characters[info.target_character].hp -= 2;
    }

    fn on_e_skill(&mut self, info: OperationContext, env: &mut GameEnvironment) {
        if self.oz.lifetime == 0 {
            env.players[info.subject_player].insert_summoned(self.oz.clone());
        }

        // This is safe because oz will only be read in one thread
        unsafe {
            let oz_raw = Arc::into_raw(self.oz.clone()) as *mut Oz;
            (*oz_raw).lifetime = 2;
        }
    }

    fn on_q_skill(&mut self, info: OperationContext, env: &mut GameEnvironment) {
        todo!()
    }
}

pub fn fischl() -> Character {
    Character {
        name: "Fischl",
        max_hp: 10,
        hp: 10,
        e_cost: 3,
        q_cost: 4,
        element: ElementType::Electro,
        handler: Arc::new(FischlHandler::default()),
        image_key: "Fischl_Character_Card.webp",
    }
}