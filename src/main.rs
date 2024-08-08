#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui::{self, CentralPanel, Layout, Ui, ViewportBuilder};

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "hello gui",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<MyAPP>::new(MyAPP::new()))
        }),
    );

    Ok(())
}

#[derive(Debug)]
enum LayoutMode {
    Center,
    LeftRight,
    StartEnd,
}

struct MyAPP {
    layout_mode: LayoutMode,
}

impl MyAPP {
    fn new() -> Self {
        Self {
            layout_mode: LayoutMode::Center,
        }
    }

    fn center_ui(ui: &mut Ui) {
        ui.with_layout(
            Layout::centered_and_justified(egui::Direction::TopDown),
            |ui| ui.label("Centered Content"),
        );
    }

    fn left_right_ui(ui: &mut Ui) {
        ui.with_layout(Layout::left_to_right(egui::Align::Min), |ui| {
            ui.label("Left Item");
            ui.separator();
            ui.label("Right item")
        });
    }

    fn start_end_ui(ui: &mut Ui) {
        ui.with_layout(Layout::left_to_right(egui::Align::Min), |ui| {
            ui.label("Start Item");
            ui.with_layout(Layout::right_to_left(egui::Align::Min), |ui| {
                ui.add_space(ui.available_width());
                ui.label("End Item");
            });
        });
    }
}

impl eframe::App for MyAPP {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Center").clicked() {
                    self.layout_mode = LayoutMode::Center;
                }
                if ui.button("Left-Right").clicked() {
                    self.layout_mode = LayoutMode::LeftRight;
                }
                if ui.button("Start-End").clicked() {
                    self.layout_mode = LayoutMode::StartEnd;
                }
            });
            match self.layout_mode {
                LayoutMode::Center => MyAPP::center_ui(ui),
                LayoutMode::LeftRight => {
                    MyAPP::left_right_ui(ui);
                }
                LayoutMode::StartEnd => {
                    MyAPP::start_end_ui(ui);
                }
            }
        });
    }
}
