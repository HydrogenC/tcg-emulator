use std::sync::Arc;
use crate::cards::ActionCard;

pub struct CardSet{
    pub cards: [Arc<dyn ActionCard>; 10]
}