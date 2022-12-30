use std::mem::{MaybeUninit, transmute};
use crate::cards::{EmptyCard, SummonedCard, SupportCard};
use crate::character::Character;
use crate::characters::fischl::fischl;
use crate::characters::yoimiya::yoimiya;
use crate::dice_set::DiceSet;

pub struct Player {
    pub dice_set: DiceSet,
    pub support_area: [Box<dyn SupportCard>; 4],
    pub summon_area: [Box<dyn SummonedCard>; 4],
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
            support_area: init_array!(Box<dyn SupportCard>, 4, Box::new(EmptyCard {})),
            summon_area: init_array!(Box<dyn SummonedCard>, 4, Box::new(EmptyCard{})),
            characters: [yoimiya(), yoimiya(), fischl()],
            active_character: 0usize,
        }
    }
}
