#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui::{self, ViewportBuilder};

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "hello android",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<MyAPP>::new(MyAPP {}))
        }),
    );

    Ok(())
}

struct MyAPP {}

impl eframe::App for MyAPP {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {}
}
