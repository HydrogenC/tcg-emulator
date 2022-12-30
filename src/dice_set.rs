use std::fmt;
use egui::Color32;
use rand::prelude::*;
use int_enum::IntEnum;

#[repr(i8)]
#[derive(PartialOrd, Ord, Clone, Copy, Eq, PartialEq, IntEnum, Debug)]
pub enum DiceType {
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

impl fmt::Display for DiceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct DiceSet {
    pub dices: [DiceType; 16],
    pub dice_count: usize,
    rand: ThreadRng,
}

impl Default for DiceSet {
    fn default() -> Self {
        DiceSet {
            dices: [DiceType::Null; 16],
            dice_count: 0,
            rand: thread_rng(),
        }
    }
}

pub enum DiceFindMode{
    Any,
    Same
}

impl DiceSet {
    fn generate_dice(&mut self) -> DiceType {
        let val = self.rand.gen::<f64>() * 8.0;
        let int_val = val.floor() as i8;
        DiceType::from_int(int_val).unwrap()
    }

    pub fn reroll_dice(&mut self, index: usize) {
        self.dices[index] = self.generate_dice();
    }

    pub fn roll_dices(&mut self) {
        self.dices.fill(DiceType::Null);

        for i in 0usize..8usize {
            self.dices[i] = self.generate_dice();
        }

        self.dice_count = 8;
    }

    pub fn find_dice(mode: DiceFindMode, ty: DiceType, num: usize){

    }
}