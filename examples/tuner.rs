use cpal::traits::{HostTrait, DeviceTrait, StreamTrait};
use egui;
use egui::{emath, Pos2, Vec2, Rect, Sense, Shape};
use std::sync::{Arc, Mutex};

#[allow(dead_code)]
pub struct App {
    input_buffer: Arc<Mutex<Vec<f32>>>
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>, input_buffer: Arc<Mutex<Vec<f32>>>) -> Self {
        Self {
            input_buffer
        }
    }
}

impl App {
    // Drawing canvas
    pub fn ui_content(&mut self, ui: &mut egui::Ui, x: f32) -> egui::Response {
        let (response, painter) =
            ui.allocate_painter(Vec2::new(ui.available_width(), 300.0), Sense::hover());

        let to_screen = emath::RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, response.rect.size()),
            response.rect,
        );

        let num_circles = 100;
        let circle_radius = 3.0;
        let margin = response.rect.width() / 10.0;
        let width = response.rect.width() - (2.0 * margin);

        let x = (x * 100.0).round() as i32;

        let circles: Vec<Shape> = (0..num_circles)
            .map(|i| {
                let color = if i < x {
                    if i > 70 {
                        egui::Color32::RED
                    } else if i > 40 {
                        egui::Color32::YELLOW
                    } else {
                        egui::Color32::GREEN
                    }
                } else {
                    egui::Color32::WHITE
                };

                let point = Pos2 {
                    x: i as f32 * (width / num_circles as f32) + margin,
                    y: response.rect.center().y
                };
                let point = to_screen.transform_pos(point);

                Shape::circle_filled(point, circle_radius, color)
            })
            .collect();

        painter.extend(circles);

        response
    }
}

impl eframe::App for App {
    // Layout the overall UI
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(&ctx, |ui| {
            ui.heading("FretBuddy: Tuner");
            ui.separator();
            ui.heading(format!("Input Buffer Size: {}", self.input_buffer.lock().unwrap().len()));

            let x = {
                let buffer = self.input_buffer.lock().unwrap();
                *buffer.last().unwrap()
            };
            ui.heading(format!("{}", x));

            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                self.ui_content(ui, x);
            });

            ctx.request_repaint();
        });
    }
}

fn main() {
    let device = cpal::default_host()
        .default_input_device()
        .unwrap();
    let config = device.default_input_config().unwrap();

    let shared_buffer = Arc::new(Mutex::new(Vec::<f32>::new()));
    let reader_buffer = Arc::clone(&shared_buffer);
    let writer_buffer = Arc::clone(&shared_buffer);

    let stream = match config.sample_format() {
        cpal::SampleFormat::I16 => todo!(),
        cpal::SampleFormat::U16 => todo!(),
        cpal::SampleFormat::F32 => device.build_input_stream(
            &config.into(),
            move |data: &[f32], _: &_| {
                let sum = data
                    .iter()
                    .fold(0.0, |acc, x| acc + x.abs());
                let sum = sum / data.len() as f32;

                let mut buffer = writer_buffer.lock().unwrap();
                buffer.push(sum);
                println!("sum = {}", sum);
            },
            move |err| {
                eprintln!("an error occurred on stream: {}", err);
            }
        ).unwrap(),
    };
    stream.play().unwrap();

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "RTA: Tuner",
        options,
        Box::new(|cc| Box::new(App::new(cc, reader_buffer)))
    );

    drop(stream);
}
