use std::cmp::Ordering;
use std::fmt;
use egui::Color32;
use rand::prelude::*;
use int_enum::IntEnum;
use crate::dice_set::ElementType::Universal;

#[repr(i8)]
#[derive(PartialOrd, Ord, Clone, Copy, Eq, PartialEq, IntEnum, Debug)]
pub enum ElementType {
    Universal = 0,
    Electro = 1,
    Hydro = 2,
    Pyro = 3,
    Cryo = 4,
    Anemo = 5,
    Geo = 6,
    Dendro = 7,
    Null = 8,
}

pub const COLORS: &[Color32] = &[
    Color32::from_rgb(255, 255, 255),
    Color32::from_rgb(211, 118, 240),
    Color32::from_rgb(28, 114, 253),
    Color32::from_rgb(226, 49, 29),
    Color32::from_rgb(152, 200, 232),
    Color32::from_rgb(51, 204, 179),
    Color32::from_rgb(207, 167, 38),
    Color32::from_rgb(123, 180, 45),
];

impl fmt::Display for ElementType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct DiceSet {
    pub dices: [ElementType; 16],
    pub dice_count: usize,
}

impl Default for DiceSet {
    fn default() -> Self {
        DiceSet {
            dices: [ElementType::Null; 16],
            dice_count: 0,
        }
    }
}

impl DiceSet {
    fn generate_dice(&mut self) -> ElementType {
        let val = thread_rng().gen::<f64>() * 8.0;
        let int_val = val.floor() as i8;
        ElementType::from_int(int_val).unwrap()
    }

    pub fn reroll_dice(&mut self, index: usize) {
        self.dices[index] = self.generate_dice();
    }

    pub fn roll_dices(&mut self) {
        self.dices.fill(ElementType::Null);

        for i in 0usize..8usize {
            self.reroll_dice(i);
        }

        self.dice_count = 8;
    }

    pub fn sort_dice(&mut self, character_types: Vec<ElementType>) {
        self.dices.sort_by(|a, b| {
            if *a == ElementType::Universal {
                return Ordering::Less;
            }

            if *b == ElementType::Universal {
                return Ordering::Greater;
            }

            // Whether there are characters of element a
            let is_a_used = character_types.contains(a);
            if is_a_used == character_types.contains(b) {
                a.int_value().cmp(&b.int_value())
            } else {
                if is_a_used {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }
        });
    }

    pub fn find_dice(&self, must_same: bool, ty: ElementType, num: usize) -> Option<Vec<usize>> {
        let mut result = Vec::<usize>::new();
        let mut remaining_dices = num;
        let mut selected_type = ty;

        if ty == ElementType::Null && must_same {
            let mut type_count = [0usize; 8];
            for i in 0usize..self.dice_count {

                let element_index = self.dices[i].int_value() as usize;
                type_count[element_index] += 1;

                let total_count = if self.dices[i] == Universal {
                    0usize
                } else {
                    type_count[0usize] + type_count[element_index]
                };

                if total_count >= num {
                    selected_type = self.dices[i];
                    break;
                }
            }
        }

        for i in (0usize..self.dice_count).rev() {

            let cond = if must_same {
                self.dices[i] == selected_type || self.dices[i] == Universal
            } else {
                true
            };

            if cond {
                result.push(i);
                remaining_dices -= 1;
                if remaining_dices == 0 {
                    break;
                }
            }
        }

        if remaining_dices == 0 {
            Some(result)
        } else {
            None
        }
    }
}