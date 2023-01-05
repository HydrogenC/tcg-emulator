use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{BufReader, Read};
use std::sync::{Arc, RwLock};
use std::sync::mpsc::{channel, Sender};
use crate::dice_set::{COLORS, ElementType};
use eframe::egui;
use egui::{Button, Color32, ImageButton, Stroke, Vec2, Visuals, Widget};
use bitvec::prelude::*;
use egui_extras::RetainedImage;
use int_enum::IntEnum;
use crate::game_environment::GameEnvironment;
use crate::messages::{Message, SkillType};

mod dice_set;
mod card_set;
mod cards;
mod game_environment;
mod player;
mod messages;
mod characters;

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };

    let (send, recv) = channel::<Message>();
    let mut app = TcgApp::new(send);
    app.initialize();

    let thread_game_env = app.game_env.clone();
    std::thread::spawn(move || {
        loop {
            let msg = recv.recv().unwrap();
            let mut env = thread_game_env.write().unwrap();

            env.game_loop(&msg);
        }
    });

    app.game_loop_recv.send(Message::TurnStart).expect("Send message failed");

    eframe::run_native(
        "tcg-emulator",
        options,
        Box::new(|_cc| Box::new(app)),
    );
}

#[derive(PartialEq, Eq)]
enum AppState {
    Operating,
    Rerolling,
    Targeting(SkillType),
}

struct TcgApp {
    game_env: Arc<RwLock<GameEnvironment>>,
    dice_selection: BitVec<u16>,
    game_loop_recv: Sender<Message>,
    app_state: AppState,
    image_dictionary: HashMap<String, Arc<RetainedImage>>,
}

impl TcgApp {
    fn new(recv: Sender<Message>) -> Self {
        TcgApp {
            game_env: Arc::new(RwLock::new(GameEnvironment::default())),
            dice_selection: BitVec::with_capacity(16),
            game_loop_recv: recv,
            app_state: AppState::Rerolling,
            image_dictionary: HashMap::new(),
        }
    }

    fn initialize(&mut self) {
        // Initialize bitset
        for _ in 0..16usize {
            self.dice_selection.push(false);
        }

        // Load resources
        let files = fs::read_dir("images/").unwrap();

        for elem in files {
            let path = elem.unwrap().path();
            let file_name = path.file_name().unwrap().to_str().unwrap();

            let f = File::open(path.as_path()).unwrap();
            let mut reader = BufReader::new(f);
            let mut buffer = Vec::new();
            reader.read_to_end(&mut buffer).unwrap();

            self.image_dictionary.insert(file_name.to_string(), Arc::new(RetainedImage::from_image_bytes(
                file_name,
                buffer.as_slice(),
            ).unwrap()));
        }
    }

    // Convert bitset to vec<usize>
    pub fn bitset_to_dice_selection(bitset: &BitVec<u16>, len: usize) -> Vec<usize> {
        let bit_val = *bitset.as_raw_slice().first().unwrap();
        let mut selected_dices = Vec::new();

        if bit_val != 0u16 {
            for i in 0usize..len {
                if bitset[i] {
                    selected_dices.push(i);
                }
            }
        }

        selected_dices
    }
}

impl eframe::App for TcgApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut env = self.game_env.write().unwrap();

        egui::CentralPanel::default().show(ctx, |ui| {
            ctx.set_pixels_per_point(2.5);
            ctx.set_visuals({
                let mut vis = Visuals::dark();
                vis.override_text_color = Some(Color32::WHITE);
                vis
            });

            ui.heading("TCG Emulator");
            ui.horizontal(|ui| {
                {
                    let player_dice_set = &mut env.player.dice_set;
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
                }

                // Update status
                if env.player.reroll_chances > 0 {
                    self.app_state = AppState::Rerolling;
                }

                if self.app_state == AppState::Rerolling {
                    if env.player.reroll_chances == 0 {
                        self.app_state = AppState::Operating;
                    } else {
                        if ui.button("Reroll dices").clicked() {
                            {
                                let player_dice_set = &mut env.player.dice_set;

                                self.game_loop_recv.send(Message::RerollDice(
                                    TcgApp::bitset_to_dice_selection(&self.dice_selection, player_dice_set.dice_count)
                                )).expect("Send message failed");
                                self.dice_selection.fill(false);
                            }
                        }
                    }
                }
            });

            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Opp");

                let player_dice_count = env.player.dice_set.dice_count;
                let object = &mut env.opponent;
                for i in 0..object.characters.len() {
                    ui.vertical(|ui| {
                        ui.label(format!("HP: {}", object.characters[i].hp));

                        let pic = self.image_dictionary[&object.characters[i].image_key.to_string()].clone();
                        let btn = ImageButton::new(
                            pic.texture_id(ctx), Vec2::new(52.5f32, 90f32));
                        if btn.ui(ui).clicked() {
                            if let AppState::Targeting(x) = &self.app_state {
                                self.game_loop_recv.send(Message::UseSkill(*x, i, TcgApp::bitset_to_dice_selection(&self.dice_selection, player_dice_count),
                                )).expect("Send message failed");
                                self.dice_selection.fill(false);
                            }
                        }
                    });
                }
            });

            ui.separator();
            ui.horizontal(|ui| {
                ui.label("You");

                let object = &mut env.player;
                for i in 0..object.characters.len() {
                    ui.vertical(|ui| {
                        ui.label(format!("HP: {}", object.characters[i].hp));

                        let pic = self.image_dictionary[&object.characters[i].image_key.to_string()].clone();
                        let btn = ImageButton::new(
                            pic.texture_id(ctx), Vec2::new(52.5f32, 90f32));
                        if btn.ui(ui).clicked() && object.active_character != i {
                            self.game_loop_recv.send(Message::ChangeActive(i))
                                .expect("Send message failed");
                        }
                    });
                }
            });

            if self.app_state != AppState::Rerolling {
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

                                self.app_state = AppState::Targeting(SkillType::NormalAttack);
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

                                self.app_state = AppState::Targeting(SkillType::ESkill);
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

                                self.app_state = AppState::Targeting(SkillType::QSkill);
                            }
                            None => {}
                        }
                    }

                    if ui.button("Declare end of turn").clicked() {
                        self.game_loop_recv.send(Message::TurnEnd).expect("Message send failed");
                        self.game_loop_recv.send(Message::TurnStart).expect("Message send failed");

                        self.dice_selection.fill(false);
                    }
                });
            }
        });
    }
}
