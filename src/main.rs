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
    player_edit_result: Vec<String>,
}

impl CompeteApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self { CompeteApp {
        player_edit_result: vec![String::new(), String::new(), String::new(), String::new()],}
    }
}

impl eframe::App for CompeteApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |cui| {
            cui.heading("Kart JSON Editor");
            for i in 0..4 {
                let response = cui.add(
                    egui::TextEdit::singleline(&mut self.player_edit_result[i])
                        .hint_text("Enter player name...")
                );
            }
            if cui.add(egui::Button::new("Submit Match")).clicked() {
                for i in 0..4 {
                    print!("{} ", self.player_edit_result[i]);
                    self.player_edit_result[i] = String::new();
                }
                println!("");
            }
        });
    }
}