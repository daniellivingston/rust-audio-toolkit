use egui::{
    Color32, Frame, Pos2, pos2
};

use crate::device_audio::{Audio, read_wav};

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
    audio: Option<Audio>
}

impl App {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        #[allow(unused_mut)]
        let mut slf = Self {
            state: State::default(),
            frequency_plot: FrequencyPlot::default(),
            picked_path: None,
            audio: None
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
            self.frequency_plot.ui(ui, &self.audio);
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

                self.audio = if let Ok(audio) = read_wav(&picked_path) {
                    self.picked_path = Some(picked_path);
                    Some(audio)
                } else {
                    self.picked_path = None;
                    None
                };
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

use egui::plot::{Plot, PlotPoints, Line};

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
    fn pure_tone_fn() -> Line {
        let values = PlotPoints::from_explicit_callback(
            move |x| {
                x.sin()
            },
            0.0..=1.0,
            100
        );
        Line::new(values)
    }

    fn ui(&mut self, ui: &mut egui::Ui, audio: &Option<Audio>) -> egui::Response {
        let color = if ui.visuals().dark_mode {
            Color32::from_additive_luminance(196)
        } else {
            Color32::from_black_alpha(240)
        };

        let _notes = super::notes();

        #[cfg(feature = "NOCOMPILE")]
        Frame::canvas(ui.style()).show(ui, |ui| {
            ui.ctx().request_repaint();

            let (_id, rect) = ui.allocate_space(ui.available_size());

            let mut shapes = vec![];

            if let Some(audio) = audio {
                let xmin = 0.0;
                let xmax = audio.duration().as_millis() as f32;
                assert!(xmin < xmax);

                let ymin = audio.data().iter().fold(std::f32::MAX, |a,b| a.min(*b));
                let ymax = audio.data().iter().fold(std::f32::MIN, |a,b| a.max(*b));
                assert!(ymin < ymax);

                let to_screen = egui::emath::RectTransform::from_to(
                    egui::Rect::from_x_y_ranges(xmin..=xmax, ymin..=ymax),
                    rect);

                let points: Vec<Pos2> = audio.data()
                    .iter()
                    .enumerate()
                    .map(|(i, &x)| to_screen * pos2(i as f32, x))
                    .collect();

                let thickness = 1.0 / 2.0 as f32;
                shapes.push(egui::epaint::Shape::line(points, egui::Stroke::new(thickness, color)));
            }

            ui.painter().extend(shapes);
        });

        Plot::new("freq_plot")
            .show(ui, |plot_ui| {
                if let Some(audio) = audio {
                    let max_pts = 100;
                    let step = audio.data().len() as usize / max_pts;

                    let points: Vec<_> = audio.data()
                        .iter()
                        .enumerate()
                        .step_by(step)
                        .map(|(i, &y)| [i as f64, y as f64])
                        .collect();

                    plot_ui.line(
                        Line::new(PlotPoints::new(points))
                            .color(color)
                    )
                } else {
                    plot_ui.line(
                        FrequencyPlot::pure_tone_fn()
                            .color(color)
                    )
                }
            })
            .response
    }
}
