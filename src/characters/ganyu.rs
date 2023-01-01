use std::sync::{Arc, Mutex};
use egui_extras::RetainedImage;
use crate::characters::character::{Character, CharacterHandler};
use crate::dice_set::ElementType;
use crate::game_environment::GameEnvironment;

struct GanyuHandler {}

impl Default for GanyuHandler {
    fn default() -> Self {
        GanyuHandler {}
    }
}

impl CharacterHandler for GanyuHandler {
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

pub fn ganyu() -> Character {
    Character {
        name: "Ganyu",
        max_hp: 10,
        hp: 10,
        e_cost: 1,
        q_cost: 3,
        element: ElementType::Cryo,
        handler: Arc::new(Mutex::new(GanyuHandler::default())),
        image: RetainedImage::from_image_bytes(
            "Ganyu",
            include_bytes!("images/Ganyu_Character_Card.webp"),
        ).unwrap(),
    }
}