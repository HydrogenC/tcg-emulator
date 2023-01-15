#[derive(PartialEq, Eq, Copy, Clone)]
pub enum SkillType {
    NormalAttack,
    ESkill,
    QSkill,
}

// Messages received from the client side
pub enum GameEvent {
    // Player index
    SetupClient(usize),
    // Player index, Character index
    ChangeActive(usize, usize),
    RoundStart,
    // Player index
    DeclareRoundEnd(usize),
    RoundEnd,
    // Player index
    TurnOf(usize),
    // Player index, Skill, Dices used
    UseSkill(usize, SkillType, Vec<usize>),
    // Player index, Card index, Character index
    UseActionCard(usize, usize, usize),
    // Player index, Dices to reroll
    RerollDice(usize, Vec<usize>),
}