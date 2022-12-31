use std::sync::{Arc, RwLock};
use std::sync::mpsc::{channel, Receiver, Sender};
use crate::dice_set::{COLORS, ElementType};
use eframe::egui;
use egui::{Button, Color32, ImageButton, Stroke, Vec2, Visuals, Widget};
use bitvec::prelude::*;
use int_enum::IntEnum;
use crate::game_environment::GameEnvironment;
use crate::messages::Message;

mod dice_set;
mod card_set;
mod cards;
mod character;
mod game_environment;
mod player;
mod characters;
mod messages;

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };

    let (send, recv) = channel::<Message>();
    let mut app = TcgApp::new(send);

    {
        let mut env = app.game_env.write().unwrap();
        env.player.dice_set.roll_dices();
        env.player.dice_set.sort_dice([ElementType::Cryo, ElementType::Electro, ElementType::Pyro]);
    }

    for _ in 0..16usize {
        app.dice_selection.push(false);
    }

    let thread_game_env = app.game_env.clone();
    std::thread::spawn(move || {
        loop {
            let msg = recv.recv().unwrap();
            let mut env = thread_game_env.write().unwrap();

            env.game_loop(&msg);
        }
    });

    eframe::run_native(
        "tcg-emulator",
        options,
        Box::new(|_cc| Box::new(app)),
    );
}

struct TcgApp {
    game_env: Arc<RwLock<GameEnvironment>>,
    dice_selection: BitVec<u16>,
    game_loop_recv: Sender<Message>,
}

impl TcgApp {
    fn new(recv: Sender<Message>) -> Self {
        TcgApp {
            game_env: Arc::new(RwLock::new(GameEnvironment::default())),
            dice_selection: BitVec::with_capacity(16),
            game_loop_recv: recv,
        }
    }
}

impl eframe::App for TcgApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut env = self.game_env.write().unwrap();

        egui::CentralPanel::default().show(ctx, |ui| {
            let player_dice_set = &mut env.player.dice_set;
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

                let object = &mut env.opponent;
                for i in 0..object.characters.len() {
                    let btn = ImageButton::new(
                        object.characters[i].image.texture_id(ctx), Vec2::new(52.5f32, 90f32));
                    btn.ui(ui);
                }
            });

            ui.separator();
            ui.horizontal(|ui| {
                ui.label("You");

                let object = &mut env.player;
                for i in 0..object.characters.len() {
                    let btn = ImageButton::new(
                        object.characters[i].image.texture_id(ctx), Vec2::new(52.5f32, 90f32));
                    if btn.ui(ui).clicked() && object.active_character != i {
                        self.game_loop_recv.send(Message::ChangeActive(i))
                            .expect("Send message failed");
                    }
                }
            });

            ui.separator();
            ui.horizontal(|ui| {
                let active_character = &env.player.characters[env.player.active_character];

                if ui.button(
                    format!("Normal Attack: 1{}+2Any", active_character.element)
                ).clicked() {
                    self.dice_selection.fill(false);

                    // Test use only
                    match env.player.dice_set
                        .find_dice(false, ElementType::Null, 3) {
                        Some(t) => {
                            for elem in t {
                                self.dice_selection.set(elem, true);
                            }
                        }
                        None => {}
                    }
                }

                if ui.button(
                    format!("E Skill: {}{}", active_character.e_cost, active_character.element)
                ).clicked() {
                    self.dice_selection.fill(false);

                    match env.player.dice_set
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

                    match env.player.dice_set
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
