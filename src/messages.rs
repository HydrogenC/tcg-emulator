use std::sync::Arc;
use crate::cards::ActionCard;

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum SkillType {
    NormalAttack,
    ESkill,
    QSkill,
}

pub enum Message {
    ChangeActive(usize),
    TurnEnd,
    TurnStart,
    UseSkill(SkillType, usize),
    UseActionCard(Arc<dyn ActionCard>, usize),
}