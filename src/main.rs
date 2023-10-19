#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use egui::Vec2;
use std::fs;
use std::collections::HashMap;

fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Option::from(
        Vec2::new(765f32, 350f32)
    );
    let _ = eframe::run_native(
        "Kart JSON Editor",
        native_options,
        Box::new(|cc| Box::new(CompeteApp::new(cc)))
    );
}

#[derive(Default)]
struct CompeteApp {
    player_edit: Vec<String>,
    score_edit: Vec<String>,
}

impl CompeteApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self { 
        CompeteApp {
            player_edit: vec![String::new(), String::new(), String::new(), String::new()],
            score_edit: vec![String::new(), String::new(), String::new(), String::new()],
        }
    }
}

fn get_match(player_entries: Vec<String>, score_entries: Vec<String>) -> HashMap<String, f32> {
    let mut result: HashMap<String, f32> = Default::default();
    assert!(player_entries.len() == score_entries.len(), "Mismatch parallel arrays for get_match(..)");
    for i in 0..player_entries.len() {
        result.insert(player_entries[i].clone(), 0f32);
        *result.get_mut(&player_entries[i]).unwrap() = score_entries[i].parse::<f32>().unwrap();
    }
    return result;
}

fn write_match(file_path: &str, mat: HashMap<String, f32>) {
    let data = fs::read_to_string(file_path).expect("Unable to read file");
    let mut content: Vec<HashMap<String, f32>> = serde_json::from_str(&data).expect("Unable to parse JSON");
    content.push(mat);
    let result = serde_json::to_string_pretty(&content).unwrap();
    let _ = fs::write(file_path, result);
}

fn process_match(player_entries: Vec<String>, score_entries: Vec<String>) {
    let mat = get_match(player_entries, score_entries);
    write_match("./content.json", mat);
}

impl eframe::App for CompeteApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |cui| {
            cui.heading("Kart JSON Editor");
            cui.separator();
            for i in 0..4 {
                cui.horizontal(|hui| {
                    hui.add(
                        egui::TextEdit::singleline(&mut self.player_edit[i])
                            .hint_text("Enter player name...")
                    );
                    hui.add(
                        egui::TextEdit::singleline(&mut self.score_edit[i])
                            .hint_text("Score...")
                    );
                });
            }
            cui.separator();
            if cui.add(egui::Button::new("Submit Match")).clicked() {
                let mut rem_index: usize = 4;
                for i in 0..4 {
                    if self.player_edit[i].len() == 0 {
                        rem_index = i;
                        break;
                    }
                }
                for _i in rem_index..4 {
                    self.player_edit.remove(rem_index);
                    self.score_edit.remove(rem_index);
                }
                process_match(self.player_edit.clone(), self.score_edit.clone());
                self.player_edit = vec![String::new(), String::new(), String::new(), String::new()];
                self.score_edit = vec![String::new(), String::new(), String::new(), String::new()];
            }
            cui.separator();
            cui.label("For One on One matches, use '1' for the winner and '0' for the loser.");
            cui.separator();
            cui.label("For Free for All matches, use their in-game score if possible: out of 60 points.");
            cui.label("Otherwise use the values of 15, 30, 45, and 60: based on placement.");
        });
    }
}