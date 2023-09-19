use eframe::egui;

fn main() {
    let native_options = eframe::NativeOptions::default();
    let x = eframe::run_native(
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

#[derive(Debug)]
struct Match {
    game_type: String,
    players: Vec<String>,
    scores: Vec<f32>,
}

fn get_match(player_entries: Vec<String>, score_entries: Vec<String>) -> Match {
    let game_type: String = match player_entries.len() {
        2 => "one_on_one".to_owned(),
        3 => "free_for_all".to_owned(),
        4 => "free_for_all".to_owned(),
        i => panic!("Unknown number of players {}", i),
    };
    let mut scores: Vec<f32> = Vec::new();
    for score_entry in &score_entries {
        scores.push(score_entry.parse::<f32>().unwrap());
    }
    Match {
        game_type,
        players: player_entries,
        scores,
    }
}

fn process_match(player_entries: Vec<String>, score_entries: Vec<String>) {
    let m = get_match(player_entries, score_entries);
    println!("{:?}", m);
}

impl eframe::App for CompeteApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
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
                for i in rem_index..4 {
                    self.player_edit.remove(rem_index);
                    self.score_edit.remove(rem_index);
                }
                process_match(self.player_edit.clone(), self.score_edit.clone());
                self.player_edit = vec![String::new(), String::new(), String::new(), String::new()];
                self.score_edit = vec![String::new(), String::new(), String::new(), String::new()];
            }
        });
    }
}