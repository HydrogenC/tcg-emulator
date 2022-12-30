use crate::cards::ActionCard;

pub struct CardSet{
    pub cards: [Box<dyn ActionCard>; 10]
}