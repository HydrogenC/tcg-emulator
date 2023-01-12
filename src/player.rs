use std::mem::{MaybeUninit, transmute};
use std::sync::Arc;
use crate::cards::{EmptyCard, SummonedCard, SupportCard};
use crate::characters::character::Character;
use crate::characters::fischl::fischl;
use crate::characters::ganyu::ganyu;
use crate::characters::yoimiya::yoimiya;
use crate::dice_set::{DiceSet, ElementType};

pub struct Player {
    pub dice_set: DiceSet,
    pub support_area: [Arc<dyn SupportCard>; 4],
    pub summoned_area: [Arc<dyn SummonedCard>; 4],
    pub characters: [Character; 3],
    pub active_character: usize,
    pub support_area_count: usize,
    pub summoned_area_count: usize,
    pub reroll_chances: usize,
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

impl Player {
    pub fn new(player_index: usize) -> Self {
        Player {
            dice_set: DiceSet::default(),
            support_area: init_array!(Arc<dyn SupportCard>, 4, Arc::new(EmptyCard {})),
            summoned_area: init_array!(Arc<dyn SummonedCard>, 4, Arc::new(EmptyCard {})),
            characters: [
                yoimiya(),
                fischl(),
                ganyu()
            ],
            active_character: 0usize,
            support_area_count: 0usize,
            summoned_area_count: 0usize,
            reroll_chances: 0usize,
        }
    }
}

impl Player {
    pub fn get_character_elements(&self) -> Vec<ElementType> {
        self.characters.iter().map(|a| a.element).collect()
    }

    pub fn insert_support(&mut self, card: Arc<dyn SupportCard>) {
        self.support_area[self.support_area_count] = card;
        self.support_area_count += 1;
    }

    pub fn remove_support(&mut self, index: usize) {
        for i in index..self.support_area_count {
            self.support_area[i] = self.support_area[i + 1].clone();
        }

        self.support_area_count -= 1;
        self.support_area[self.support_area_count] = Arc::new(EmptyCard {});
    }

    pub fn insert_summoned(&mut self, card: Arc<dyn SummonedCard>) {
        self.summoned_area[self.summoned_area_count] = card;
        self.support_area_count += 1;
    }

    pub fn remove_summoned(&mut self, index: usize) {
        for i in index..self.summoned_area_count {
            self.summoned_area[i] = self.summoned_area[i + 1].clone();
        }

        self.summoned_area_count -= 1;
        self.summoned_area[self.summoned_area_count] = Arc::new(EmptyCard {});
    }
}
