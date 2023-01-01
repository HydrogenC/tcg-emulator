use std::sync::{Arc, Mutex};
use egui_extras::RetainedImage;
use crate::characters::character::{Character, CharacterHandler};
use crate::dice_set::ElementType;
use crate::game_environment::GameEnvironment;

struct FischlHandler {}

impl Default for FischlHandler {
    fn default() -> Self {
        FischlHandler {}
    }
}

impl CharacterHandler for FischlHandler {
    fn on_normal_attack(&mut self, me: usize, target: usize, env: &mut GameEnvironment) {
        todo!()
    }

    fn on_e_skill(&mut self, me: usize, target: usize, env: &mut GameEnvironment) {
        todo!()
    }

    fn on_q_skill(&mut self, me: usize, target: usize, env: &mut GameEnvironment) {
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
        handler: Arc::new(Mutex::new(FischlHandler::default())),
        image: RetainedImage::from_image_bytes(
            "Fischl",
            include_bytes!("images/Fischl_Character_Card.webp"),
        ).unwrap(),
    }
}