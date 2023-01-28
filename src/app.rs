use egui::{
    Response,
    Color32,
    plot::{Plot, Legend, PlotPoints, LineStyle, Line}
};

/// Peristent app state.
#[derive(Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct State {
    noise_gate: u64
}

pub struct App {
    state: State,
    frequency_plot: FrequencyPlot
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        #[allow(unused_mut)]
        let mut slf = Self {
            state: State::default(),
            frequency_plot: FrequencyPlot::default()
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
    fn side_panel(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        ui.heading("Frequency Analysis");
        ui.add(egui::Slider::new(&mut self.state.noise_gate, 0..=100).text("Noise gate"));
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

#[derive(PartialEq)]
struct FrequencyPlot { }

impl Default for FrequencyPlot {
    fn default() -> Self {
        Self { }
    }
}

impl FrequencyPlot {
    fn ui(&mut self, ui: &mut egui::Ui) -> Response {
        ui.ctx().request_repaint();
        ui.heading("Hello, Frequency Plot!");

        Plot::new("frequency_plot")
            .legend(Legend::default())
            .show(ui, |plot_ui| {
                let n = 50;
                let pts: PlotPoints = (0..n)
                    .map(|i| {
                        let x = i as f64;
                        [ x, x ]
                    })
                    .collect();

                Line::new(pts)
                    .color(Color32::from_rgb(100, 200, 100))
                    .style(LineStyle::Solid)
                    .name("line")
            })
            .response
    }
}

