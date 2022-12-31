use crate::cards::ActionCard;

pub enum Message {
    ChangeActive(usize),
    UseNormalAttack,
    UseESkill,
    UseQSkill,
    UseActionCard(Box<dyn ActionCard>),
}