use crate::game_environment::GameEnvironment;
use crate::operation_context::OperationContext;

pub trait ActionCard: Send + Sync {
    fn use_card(&self, info: OperationContext, env: &mut GameEnvironment) {}
}

pub trait SupportCard: Send + Sync {
    fn on_created(&self, subject_player: usize, env: &mut GameEnvironment) {}
    fn on_turn_start(&self, subject_player: usize, env: &mut GameEnvironment) {}
    fn on_turn_end(&self, subject_player: usize, env: &mut GameEnvironment) {}
}

pub trait SummonedCard: Send + Sync {
    fn on_attacked(&self, subject_player: usize, env: &mut GameEnvironment) {}
    fn on_turn_end(&self, subject_player: usize, env: &mut GameEnvironment) {}
    fn remaining_uses(&self) -> usize;
}

#[derive(Copy, Clone)]
pub struct EmptyCard {}

impl ActionCard for EmptyCard {}

impl SupportCard for EmptyCard {}

impl SummonedCard for EmptyCard {
    fn remaining_uses(&self) -> usize {
        1
    }
}
