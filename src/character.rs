use crate::game_environment::GameEnvironment;

pub trait Character {
    fn on_normal_attack(&self, target: usize, env: &mut GameEnvironment) {}
    fn on_e_skill(&self, target: usize, env: &mut GameEnvironment) {}
    fn on_q_skill(&self, target: usize, env: &mut GameEnvironment) {}
}
