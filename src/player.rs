use std::mem::{MaybeUninit, transmute};
use std::sync::Arc;
use crate::cards::{EmptyCard, SummonedCard, SupportCard};
use crate::characters::character::Character;
use crate::characters::fischl::fischl;
use crate::characters::ganyu::ganyu;
use crate::characters::yoimiya::yoimiya;
use crate::dice_set::DiceSet;

pub struct Player {
    pub dice_set: DiceSet,
    pub support_area: [Arc<dyn SupportCard>; 4],
    pub summon_area: [Arc<dyn SummonedCard>; 4],
    pub characters: [Character; 3],
    pub active_character: usize,
}

macro_rules! init_array {
    ($elem:ty, $len:expr, $def:expr) => {
 // macro expand to this code
        unsafe {
            let mut arr: [MaybeUninit<$elem>; $len] = MaybeUninit::uninit().assume_init();
            for elem in &mut arr {
                elem.write($def);
            }
            transmute::<_, [$elem; $len]>(arr)
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Player {
            dice_set: DiceSet::default(),
            support_area: init_array!(Arc<dyn SupportCard>, 4, Arc::new(EmptyCard {})),
            summon_area: init_array!(Arc<dyn SummonedCard>, 4, Arc::new(EmptyCard{})),
            characters: [yoimiya(), ganyu(), fischl()],
            active_character: 0usize,
        }
    }
}
