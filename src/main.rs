//! 主程序入口
//! 使用模块化架构的 eframe 应用

use eframe::egui;
use std::sync::Arc;

// 导入模块
mod app;
mod config;
mod database;
mod ui;
mod utils;

// 使用模块中的类型
use app::MyApp;
use ui::UIRenderer;

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let current_time = ctx.input(|i| i.time);

        // 更新状态消息
        self.update_status_message(ctx);

        // 简单的菜单关闭检测
        // 只在用户点击外部区域或按 ESC 键时关闭菜单
        if self.active_menu.is_some() {
            // ESC 键关闭菜单
            if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
                self.active_menu = None;
                self.menu_open_time = None;
                self.menu_click_pos = None;
            }
        }

        // 使用模块化的 UI 渲染
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.vertical(|ui| {
                UIRenderer::render_top_menu(ui, self, ctx);
                // ui.separator();
                // ui.horizontal(|ui| {
                //     ui.label("状态栏");
                // });
            });
        });

        // 渲染下拉菜单
        UIRenderer::render_dropdown_menu(self, ctx);

        // 渲染底部状态栏
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            UIRenderer::render_status_bar(ui, self);
        });

        // 渲染侧边栏
        egui::SidePanel::left("left_panel").show(ctx, |ui| {
            UIRenderer::render_sidebar(ui, self);
        });

        // 渲染主内容区域
        egui::CentralPanel::default().show(ctx, |ui| {
            UIRenderer::render_main_content(ui, self);
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let icon = include_bytes!("../assets/icons/h.png");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_icon(eframe::icon_data::from_png_bytes(icon).expect("Failed to load icon")),
        ..Default::default()
    };

    eframe::run_native(
        "hi here!",
        options,
        Box::new(|cc| {
            setup_fonts(&cc.egui_ctx);
            Ok(Box::new(MyApp::default()))
        }),
    )
}

fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "mm".to_owned(),
        Arc::new(egui::FontData::from_static(include_bytes!(
            "D:\\code\\gui\\e\\assets\\fonts\\mm.ttf"
        ))),
    );
    fonts.font_data.insert(
        "dl".to_owned(),
        Arc::new(egui::FontData::from_static(include_bytes!(
            "D:\\code\\gui\\e\\assets\\fonts\\dl.ttf"
        ))),
    );

    for font_family in [egui::FontFamily::Proportional, egui::FontFamily::Monospace] {
        if let Some(families) = fonts.families.get_mut(&font_family) {
            families.insert(0, "mm".to_owned());
        }
    }

    ctx.set_fonts(fonts);
}
