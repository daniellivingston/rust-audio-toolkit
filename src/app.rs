use egui::{
    Color32, Frame, Pos2, pos2
};

use super::read_wav;

/// Peristent app state.
#[derive(Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct State {
    noise_gate: u64
}

pub struct App {
    state: State,
    frequency_plot: FrequencyPlot,
    picked_path: Option<String>,
}

impl App {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        #[allow(unused_mut)]
        let mut slf = Self {
            state: State::default(),
            frequency_plot: FrequencyPlot::default(),
            picked_path: None
        };

        // Load previous app state (if any).
        #[cfg(feature = "persistence")]
        if let Some(storage) = cc.storage {
            if let Some(state) = eframe::get_value(storage, eframe::APP_KEY) {
                slf.state = state;
            }
        }

        slf
    }
}

impl eframe::App for App {
    /// Called by the frame work to save state before shutdown.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self.state);
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            egui::trace!(ui);
            ui.horizontal_wrapped(|ui| {
                ui.visuals_mut().button_frame = false;
                self.toolbar(ui, frame);
            });
        });

        egui::SidePanel::left("side_panel").show(&ctx, |ui| {
            self.side_panel(ui, frame);
        });

        egui::CentralPanel::default().show(&ctx, |ui| {
            self.frequency_plot.ui(ui);
        });
    }
}

impl App {
    fn side_panel(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("Frequency Analysis");

        ui.separator();

        ui.label("Input Audio:");
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.frequency_plot.device, Enum::First, "Default (Demo)");
            ui.selectable_value(&mut self.frequency_plot.device, Enum::Second, "Input Device");
            ui.selectable_value(&mut self.frequency_plot.device, Enum::Third, "Audio File");
        });

        if ui.button("Open file...").clicked() {
            if let Some(path) = rfd::FileDialog::new().pick_file() {
                let picked_path = path.display().to_string();

                let _audio = read_wav(&picked_path);
                self.picked_path = Some(picked_path);
            }
        }

        if let Some(picked_path) = &self.picked_path {
            ui.horizontal(|ui| {
                let filename = picked_path.split("/").last().unwrap_or("???");
                ui.label("Selected file:");
                ui.monospace(filename);
            });
        }

        ui.separator();

        ui.label("Noise Gate:");
        ui.add(egui::Slider::new(&mut self.state.noise_gate, 0..=100));

    }

    fn toolbar(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        ui.menu_button("File", |ui| {
            ui.set_min_width(200.0);
            ui.style_mut().wrap = Some(false);

            if ui.add(egui::Button::new("Quit"))
                 .clicked()
            {
                frame.close();
            }
        });

        ui.separator();

        egui::widgets::global_dark_light_mode_switch(ui);
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug, PartialEq)]
enum Enum {
    First,
    Second,
    Third
}

impl Default for Enum {
    fn default() -> Self {
        Self::First
    }
}

#[derive(Default)]
struct FrequencyPlot {
    device: Enum
}

impl FrequencyPlot {
    fn ui(&mut self, ui: &mut egui::Ui) {
        let color = if ui.visuals().dark_mode {
            Color32::from_additive_luminance(196)
        } else {
            Color32::from_black_alpha(240)
        };

        let _notes = super::notes();

        Frame::canvas(ui.style()).show(ui, |ui| {
            ui.ctx().request_repaint();

            let (_id, rect) = ui.allocate_space(ui.available_size());
            let to_screen = egui::emath::RectTransform::from_to(
                egui::Rect::from_x_y_ranges(0.0..=1.0, -1.0..=1.0),
                rect);

            let time = ui.input().time;

            let n = 120;
            let speed = 1.5;
            let mode = 2.0;

            let mut shapes = vec![];

            let points: Vec<Pos2> = (0..=n)
                .map(|i| {
                    let t = i as f64 / (n as f64);
                    let amp = (time * speed * mode).sin() / mode;
                    let y = amp * (t * std::f64::consts::TAU / 2.0 * mode).sin();
                    to_screen * pos2(t as f32, y as f32)
                })
                .collect();

            let thickness = 1.0 / mode as f32;
            shapes.push(egui::epaint::Shape::line(points, egui::Stroke::new(thickness, color)));

            ui.painter().extend(shapes);
        });
    }
}
