use cpal::{HostId, traits::{HostTrait, DeviceTrait}};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: f32,

    #[serde(skip)]
    host: cpal::Host,

    #[serde(skip)]
    input_device: Option<cpal::Device>,

    #[serde(skip)]
    all_input_devices: Vec<cpal::Device>,

    // temporary workaround until enum-like behavior is generated for input_device
    input_device_name: String,
}

impl Default for TemplateApp {
    fn default() -> Self {
        let host = cpal::default_host();
        let all_input_devices: Vec<cpal::Device> = host.input_devices().unwrap().collect();

        Self {
            label: "Hello World!".to_owned(),
            value: 2.7,
            host: host,
            input_device: None,
            all_input_devices: all_input_devices,
            input_device_name: String::from("N/A"),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    fn query_input_devices(&self) -> Vec<cpal::Device> {
        self.host.input_devices().unwrap().collect()
    }

    fn get_input_device_name(&self) -> String {
        if let Some(device) = &self.input_device {
            device
                .name()
                .unwrap_or(String::from("Unknown Device"))
        } else {
            String::from("N/A")
        }
    }

    fn toolbar(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::widgets::global_dark_light_mode_switch(ui);

        ui.separator();

        egui::ComboBox::from_label("Input Device")
            .selected_text(format!("{:?}", self.get_input_device_name()))
            .show_ui(ui, |ui| {
                self.all_input_devices.iter().for_each(|device| {
                    ui.selectable_value(
                        &mut self.input_device_name,
                        device.name().unwrap_or(String::from("Unknown Device")),
                        device.name().unwrap_or(String::from("Unknown Device")));
                });
            });
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // let Self { label, value } = self;

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(&ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.close();
                    }
                });
            });
        });

        egui::TopBottomPanel::top("wrap_app_top_bar").show(&ctx, |ui| {
            egui::trace!(ui);
            ui.horizontal_wrapped(|ui| {
                ui.visuals_mut().button_frame = false;
                self.toolbar(ui, frame);
            });
        });

        egui::SidePanel::left("side_panel").show(&ctx, |ui| {
            ui.heading("Side Panel");

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to(
                        "eframe",
                        "https://github.com/emilk/egui/tree/master/crates/eframe",
                    );
                    ui.label(".");
                });
            });
        });

        egui::CentralPanel::default().show(&ctx, |ui| {
            ui.heading("eframe template");
            ui.hyperlink("https://github.com/emilk/eframe_template");
            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/master/",
                "Source code."
            ));
            egui::warn_if_debug_build(ui);
        });
    }
}
