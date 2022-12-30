use egui_extras::RetainedImage;
use crate::character::{Character, CharacterHandler};
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
    fn on_normal_attack(&self, me: usize, target: usize, env: &mut GameEnvironment) {
        todo!()
    }

    fn on_e_skill(&self, me: usize, target: usize, env: &mut GameEnvironment) {
        todo!()
    }

    fn on_q_skill(&self, me: usize, target: usize, env: &mut GameEnvironment) {
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
        handler: Box::new(YoimiyaHandler::default()),
        image: RetainedImage::from_image_bytes(
            "Yoimiya",
            include_bytes!("images/Yoimiya_Character_Card.webp"),
        ).unwrap(),
    }
}