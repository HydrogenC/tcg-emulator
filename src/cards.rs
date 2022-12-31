use crate::game_environment::GameEnvironment;

pub trait ActionCard: Send + Sync {
    fn use_card(&self, target: usize, env: &mut GameEnvironment) {}
}

pub trait SupportCard: Send + Sync {
    fn on_created(&self, env: &mut GameEnvironment) {}
    fn on_turn_start(&self, env: &mut GameEnvironment) {}
    fn on_turn_end(&self, env: &mut GameEnvironment) {}
}

pub trait SummonedCard: Send + Sync {
    fn on_turn_end(&self, env: &mut GameEnvironment) {}
}

#[derive(Copy, Clone)]
pub struct EmptyCard {}

impl ActionCard for EmptyCard {}

impl SupportCard for EmptyCard {}

impl SummonedCard for EmptyCard {}
