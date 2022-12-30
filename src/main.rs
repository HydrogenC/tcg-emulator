use crate::dice_set::{COLORS, DiceSet};
use eframe::egui;
use egui::{Button, Color32, Vec2, Widget};
use int_enum::IntEnum;
use crate::game_environment::GameEnvironment;

mod dice_set;
mod card_set;
mod cards;
mod character;
mod game_environment;
mod player;

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };

    let mut app = TcgApp::default();
    app.game_env.player.dice_set.roll_dices();
    app.game_env.player.dice_set.dices.sort();

    eframe::run_native(
        "tcg-emulator",
        options,
        Box::new(|_cc| Box::new(app)),
    );
}

struct TcgApp {
    game_env: GameEnvironment,
}

impl Default for TcgApp {
    fn default() -> Self {
        TcgApp {
            game_env: GameEnvironment::default()
        }
    }
}

impl eframe::App for TcgApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let player_dice_set=&mut self.game_env.player.dice_set;
            ctx.set_pixels_per_point(3.0);

            ui.heading("TCG Emulator");
            ui.horizontal(|ui| {
                for i in 0usize..player_dice_set.dice_count {
                    let btn = Button::new("C")
                        .min_size(Vec2::new(40f32, 40f32))
                        .fill(COLORS[player_dice_set.dices[i].int_value() as usize]);
                    btn.ui(ui);
                }
            });
            if ui.button("Reroll dices").clicked() {
                player_dice_set.roll_dices();
                player_dice_set.dices.sort();
            }
        });
    }
}
