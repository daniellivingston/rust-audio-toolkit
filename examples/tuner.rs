use egui;

#[allow(dead_code)]
pub struct App {
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self { }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(&ctx, |ui| {
            ui.heading("FooBar");
            ui.separator();
            ui.heading("FooBar2");
        });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native("RTA: Tuner", options, Box::new(|cc| Box::new(App::new(cc))));
}
