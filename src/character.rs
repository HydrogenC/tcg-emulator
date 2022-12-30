use crate::dice_set::ElementType;
use crate::game_environment::GameEnvironment;

pub trait CharacterHandler {
    fn on_normal_attack(&self, me: usize, target: usize, env: &mut GameEnvironment);
    fn on_e_skill(&self, me: usize, target: usize, env: &mut GameEnvironment);
    fn on_q_skill(&self, me: usize, target: usize, env: &mut GameEnvironment);
}

pub struct Character {
    pub(crate) name: &'static str,
    pub(crate) max_hp: usize,
    pub(crate) hp: usize,
    pub(crate) e_cost: usize,
    pub(crate) q_cost: usize,
    pub(crate) element: ElementType,
    pub(crate) handler: Box<dyn CharacterHandler>,
}
