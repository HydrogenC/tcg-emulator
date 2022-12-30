use std::borrow::Borrow;
use crate::dice_set::{COLORS, DiceSet, ElementType};
use eframe::egui;
use egui::{Button, Color32, ImageButton, Stroke, Vec2, Visuals, Widget};
use bitvec::prelude::*;
use int_enum::IntEnum;
use crate::game_environment::GameEnvironment;
use crate::player::Player;

mod dice_set;
mod card_set;
mod cards;
mod character;
mod game_environment;
mod player;
mod characters;

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };

    let mut app = TcgApp::default();
    app.game_env.player.dice_set.roll_dices();
    app.game_env.player.dice_set.sort_dice([ElementType::Cryo, ElementType::Electro, ElementType::Pyro]);
    for _ in 0..16usize {
        app.dice_selection.push(false);
    }

    eframe::run_native(
        "tcg-emulator",
        options,
        Box::new(|_cc| Box::new(app)),
    );
}

struct TcgApp {
    game_env: GameEnvironment,
    dice_selection: BitVec<u16>,
}

impl Default for TcgApp {
    fn default() -> Self {
        TcgApp {
            game_env: GameEnvironment::default(),
            dice_selection: BitVec::with_capacity(16),
        }
    }
}

impl eframe::App for TcgApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let player_dice_set = &mut self.game_env.player.dice_set;
            ctx.set_pixels_per_point(3.0);
            ctx.set_visuals({
                let mut vis = Visuals::dark();
                vis.override_text_color = Some(Color32::WHITE);
                vis
            });

            ui.heading("TCG Emulator");
            ui.horizontal(|ui| {
                for i in 0usize..player_dice_set.dice_count {
                    let stroke = if self.dice_selection[i] {
                        Stroke { width: 2.5f32, color: Color32::WHITE }
                    } else {
                        Stroke { width: 1f32, color: Color32::WHITE }
                    };
                    let btn = Button::new("")
                        .min_size(Vec2::new(25f32, 25f32))
                        .fill(COLORS[player_dice_set.dices[i].int_value() as usize])
                        .stroke(stroke);
                    if btn.ui(ui).clicked() {
                        let orig = *self.dice_selection.get(i).unwrap();
                        self.dice_selection.set(i, !orig);
                    }
                }

                if ui.button("Reroll dices").clicked() {
                    let bit_val = *self.dice_selection.as_raw_slice().first().unwrap();
                    if bit_val == 0u16 {
                        player_dice_set.roll_dices();
                    } else {
                        for i in 0usize..player_dice_set.dice_count {
                            if self.dice_selection[i] {
                                player_dice_set.reroll_dice(i);
                            }

                            self.dice_selection.set(i, false);
                        }
                    }

                    player_dice_set.sort_dice([ElementType::Cryo, ElementType::Electro, ElementType::Pyro]);
                }
            });

            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Opp");

                let object = &mut self.game_env.opponent;
                for i in 0..object.characters.len() {
                    let btn = ImageButton::new(
                        object.characters[i].image.texture_id(ctx), Vec2::new(52.5f32, 90f32));
                    btn.ui(ui);
                }
            });

            ui.separator();
            ui.horizontal(|ui| {
                ui.label("You");

                let object = &mut self.game_env.player;
                for i in 0..object.characters.len() {
                    let btn = ImageButton::new(
                        object.characters[i].image.texture_id(ctx), Vec2::new(52.5f32, 90f32));
                    if btn.ui(ui).clicked() {
                        object.active_character = i;
                    }
                }
            });

            // Renew borrow
            let player_dice_set = &mut self.game_env.player.dice_set;
            ui.separator();
            ui.horizontal(|ui| {
                let active_character = &self.game_env.player.characters[self.game_env.player.active_character];

                let _ = ui.button(
                    format!("Normal Attack: 1{}+2Any", active_character.element));
                if ui.button(
                    format!("E Skill: {}{}", active_character.e_cost, active_character.element)
                ).clicked() {
                    self.dice_selection.fill(false);

                    match player_dice_set
                        .find_dice(true, active_character.element, active_character.e_cost) {
                        Some(t) => {
                            for elem in t {
                                self.dice_selection.set(elem, true);
                            }
                        }
                        None => {}
                    }
                }

                if ui.button(
                    format!("Q Skill: {}{}", active_character.q_cost, active_character.element)
                ).clicked() {
                    self.dice_selection.fill(false);

                    match player_dice_set
                        .find_dice(true, active_character.element, active_character.q_cost) {
                        Some(t) => {
                            for elem in t {
                                self.dice_selection.set(elem, true);
                            }
                        }
                        None => {}
                    }
                }
            });
        });
    }
}
