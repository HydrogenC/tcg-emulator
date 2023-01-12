#[derive(PartialEq, Eq, Copy, Clone)]
pub enum SkillType {
    NormalAttack,
    ESkill,
    QSkill,
}

pub enum GameEvents {
    RequestCharacterList,
    ChangeActive(usize),
    TurnStart,
    TurnEnd,
    UseSkill(SkillType, Vec<usize>),
    UseActionCard(usize, usize),
    RerollDice(Vec<usize>),
    TerminateGame,
}