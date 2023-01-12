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
    pub(crate) name: &'static str,
    pub(crate) max_hp: usize,
    pub(crate) hp: usize,
    pub(crate) e_cost: usize,
    pub(crate) q_cost: usize,
    pub(crate) element: ElementType,
    pub(crate) handler: Arc<dyn CharacterHandler>,
    pub(crate) image_key: &'static str,
}
