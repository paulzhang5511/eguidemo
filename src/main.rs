#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui::{self, CentralPanel, Layout, Ui, ViewportBuilder};

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([620.0, 340.0]),
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
    CenterWithTwoItems,
}

struct MyAPP {
    layout_mode: LayoutMode,
}

impl MyAPP {
    fn new() -> Self {
        Self {
            layout_mode: LayoutMode::StartEnd,
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
                ui.label("End Item");
                ui.add_space(ui.available_width());
            });
        });
    }

    fn center_with_two_items(ui: &mut Ui) {
        ui.horizontal_centered(|ui| {
            MyAPP::centerer(ui, |ui| {
                ui.label("Item 1");
                ui.add_space(6.0);
                ui.label("Item 2");
            });
        });
    }

    /// 这个辅助函数用于在 egui 框架中水平居中任意小部件。
    /// 它通过在渲染后测量小部件的宽度，并在下一帧使用该偏移量来实现水平居中。
    fn centerer(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) {
        ui.horizontal(|ui| {
            let id = ui.id().with("_centerer");
            let last_width: Option<f32> = ui.memory_mut(|mem| mem.data.get_temp(id));
            // 如果有上一次的宽度记录，添加空白空间以居中内容
            if let Some(last_width) = last_width {
                ui.add_space((ui.available_width() - last_width) / 2.0);
            }
            // 捕获当前内容的宽度
            let res = ui
                .scope(|ui| {
                    add_contents(ui);
                })
                .response;
            let width = res.rect.width();
            // 将当前宽度存储在内存中，以便下一帧使用
            ui.memory_mut(|mem| mem.data.insert_temp(id, width));

            // 如果宽度发生了变化，请求重绘
            match last_width {
                None => ui.ctx().request_repaint(),
                Some(last_width) if last_width != width => ui.ctx().request_repaint(),
                Some(_) => {}
            }
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
                if ui.button("CenterWithTwoItems").clicked() {
                    self.layout_mode = LayoutMode::CenterWithTwoItems;
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
                LayoutMode::CenterWithTwoItems => {
                    MyAPP::center_with_two_items(ui);
                }
            }
        });
    }
}
