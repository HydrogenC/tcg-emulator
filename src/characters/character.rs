use std::sync::Arc;
use crate::operation_context::OperationContext;
use crate::dice_set::ElementType;
use crate::game_environment::GameEnvironment;

pub trait CharacterHandler: Send + Sync {
    fn on_normal_attack(&mut self, info: OperationContext, env: &mut GameEnvironment);
    fn on_e_skill(&mut self, info: OperationContext, env: &mut GameEnvironment);
    fn on_q_skill(&mut self, info: OperationContext, env: &mut GameEnvironment);
}

pub struct Character {
    pub name: &'static str,
    pub max_hp: usize,
    pub hp: usize,
    pub e_cost: usize,
    pub q_cost: usize,
    pub element: ElementType,
    pub handler: Arc<dyn CharacterHandler>
}
