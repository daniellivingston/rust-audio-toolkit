use egui;

use rta::device_audio::Audio;

/// Peristent app state.
#[derive(Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct State {
}

#[allow(dead_code)]
pub struct App {
    state: State,
    frequency_plot: FrequencyPlot,
    picked_path: Option<String>,
    audio: Option<Audio<i32>>
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

            ui.separator();
            ui.heading("Frequency Analysis");
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
        ui.vertical_centered_justified(|ui| {
            ui.menu_button(egui::RichText::from("Import Audio +").size(15.0), |ui| {
                if ui.button("Open file...").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        let picked_path = path.display().to_string();

                        if let Ok(audio) = Audio::<i32>::from_wav(&picked_path) {
                            let filename = picked_path.split("/").last().unwrap_or("???");
                            self.frequency_plot.add(audio, filename);
                        }
                    }
                }

                if ui.button("Generate sine wave...").clicked() {
                    if let Ok(audio) = Audio::<i32>::from_freq(400.0, 1.0) {
                        self.frequency_plot.add(audio, "440.0 Hz @ 1.0 s");
                    }
                }
            });
        });

        ui.separator();

        ui.label("Max points:");
        ui.add(egui::Slider::new(
               &mut self.frequency_plot.max_pts,
               100..=20_000));

        ui.separator();

        self.frequency_plot.lines.iter().for_each(|line| {
            // ui.checkbox(&mut line.enabled, text);
            ui.monospace(line.name.clone());
            ui.separator();
        });
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

struct AudioPlot {
    // audio: Audio<i32>,
    points: Vec<[f64; 2]>,
    name: String,
    fft: Vec<f32>
}

impl AudioPlot {
    pub fn new(audio: Audio<i32>, name: String) -> Self {
        let fft = audio.fft();
        let points: Vec<_> = audio.data()
                .iter()
                .enumerate()
                .map(|(i, y)| [i as f64, *y as f64])
                .collect();

        Self {
            points: points,
            name: name,
            fft: fft
        }
    }
}

struct FrequencyPlot {
    max_pts: usize,
    lines: Vec<AudioPlot>,
    normalized: bool,
    log_scale: bool,
    data_aspect_fft: f32
}

impl Default for FrequencyPlot {
    fn default() -> Self {
        Self {
            max_pts: 10_000,
            lines: Vec::default(),
            normalized: false,
            log_scale: false,
            data_aspect_fft: 0.1
        }
    }
}

impl FrequencyPlot {
    pub fn add(&mut self, audio: Audio<i32>, name: &str) {
        self.lines.push(AudioPlot::new(audio, String::from(name)));
    }

    fn ui(&mut self, ui: &mut egui::Ui) -> egui::Response {
        ui.horizontal(|ui| {
            ui.checkbox(&mut self.normalized, "Normalize");
            ui.checkbox(&mut self.log_scale, "FFT Log Scale (Y)");
            ui.add(
                egui::DragValue::new(&mut self.data_aspect_fft)
                    .speed(0.1)
                    .clamp_range(0.01..=1.5)
                    .prefix("aspect: ")
            )
        });
        ui.vertical(|ui| {
            let plot_height = 0.7 * ui.available_size().y;

            let step = 100;

            Plot::new("fft_plot")
                .height(plot_height)
                .data_aspect(self.data_aspect_fft)
                .show(ui, |plot_ui| {
                    self.lines.iter().for_each(|line| {
                        let mut pts: Vec<_> = line
                            .fft
                            .iter()
                            .enumerate()
                            .step_by(step)
                            .map(|(i, &y)| [i as f64, y as f64])
                            .collect();

                        if self.log_scale {
                            pts
                                .iter_mut()
                                .for_each(|xy| xy[1] = xy[1].log10());
                        }

                        if self.normalized {
                            // Get the max value of the y-axis
                            let max_y = pts
                                .iter()
                                .fold(
                                    std::f64::MIN,
                                    |y_max, xy| y_max.max(xy[1])
                                );

                            // Normalize all y-axis elements with the max value
                            pts
                                .iter_mut()
                                .for_each(|xy| xy[1] /= max_y);
                        }

                        let pts = PlotPoints::new(pts);
                        plot_ui.line(Line::new(pts));
                    });
                });

            ui.separator();

            Plot::new("freq_plot")
                .data_aspect(1.0)
                .show(ui, |plot_ui| {
                    self.lines.iter().for_each(|line| {
                        let pts = line
                            .points
                            .iter()
                            .step_by(100)
                            .map(|&xy| [xy[0], xy[1]])
                            .collect();
                        let pts = PlotPoints::new(pts);
                        plot_ui.line(Line::new(pts));
                    });
                });
        })
        .response
    }
}

// ----------------------------------------------------------------------------

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native("RTA",
        options,
        Box::new(|cc| Box::new(App::new(cc))));
}
