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
    Color32::from_rgb(255, 0, 255),
    Color32::from_rgb(0, 0, 255),
    Color32::from_rgb(255, 0, 0),
    Color32::from_rgb(0, 255, 255),
    Color32::from_rgb(200, 200, 255),
    Color32::from_rgb(255, 255, 0),
    Color32::from_rgb(0, 255, 0),
];

impl fmt::Display for DiceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

pub struct DiceSet {
    pub dices: [DiceType; 16],
    pub dice_count: usize,
    rand: ThreadRng,
}

impl DiceSet {
    pub fn new() -> Self {
        DiceSet {
            dices: [DiceType::Null; 16],
            dice_count: 0,
            rand: thread_rng(),
        }
    }

    fn generate_dice(&mut self) -> DiceType {
        let val = self.rand.gen::<f64>() * 8.0;
        let int_val = val.floor() as i8;
        DiceType::from_int(int_val).unwrap()
    }

    pub fn roll_dices(&mut self) {
        self.dices.fill(DiceType::Null);

        for i in 0usize..8usize {
            self.dices[i] = self.generate_dice();
        }

        self.dice_count = 8;
    }
}