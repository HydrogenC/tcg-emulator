use crate::character::Character;
use crate::game_environment::GameEnvironment;

pub trait ActionCard {
    fn use_card(&self, target: usize, env: &mut GameEnvironment) {}
}

pub trait SupportCard {
    fn on_created(&self, env: &mut GameEnvironment) {}
    fn on_turn_start(&self, env: &mut GameEnvironment) {}
    fn on_turn_end(&self, env: &mut GameEnvironment) {}
}

pub trait SummonedCard {
    fn on_turn_end(&self, env: &mut GameEnvironment) {}
}

#[derive(Copy, Clone)]
pub struct EmptyCard {}

impl ActionCard for EmptyCard {}

impl SupportCard for EmptyCard {}

impl SummonedCard for EmptyCard {}
