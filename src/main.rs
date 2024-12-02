use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    
    eframe::run_native(
        "Rust Advent",
        options,
        Box::new(|_cc| Box::new(AdventApp::default()))
    )
}

struct AdventApp {
    chocolate: bool,
}

impl Default for AdventApp {
    fn default() -> Self {
        Self {
            chocolate: false,
        }
    }
}

impl eframe::App for AdventApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui_chocolate(ui, &mut self.chocolate);
        });
    }
}

fn ui_chocolate(ui: &mut egui::Ui, chocolate: &mut bool) {  // Changed parameter type to &mut bool
    ui.horizontal(|ui| {
        if ui.button("Toggle").clicked() {
            *chocolate = !*chocolate;  // Flip the boolean value
        }
        ui.label(chocolate.to_string());  // Convert bool to string for display
    });
}