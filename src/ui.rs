//! UI 模块
//! 处理界面渲染和用户交互

use crate::app::{MenuType, MyApp};
use eframe::egui;

/// UI 渲染器
pub struct UIRenderer;

impl UIRenderer {
    /// 渲染顶部菜单栏
    pub fn render_top_menu(ui: &mut egui::Ui, app: &mut MyApp, ctx: &egui::Context) {
        ui.horizontal(|ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                let menu_items = [
                    (MenuType::File, "文件(F)"),
                    (MenuType::Edit, "编辑(E)"),
                    (MenuType::Navigate, "导航(N)"),
                    (MenuType::Search, "搜索(A)"),
                    (MenuType::SqlEditor, "SQL编辑器"),
                    (MenuType::Database, "数据库(D)"),
                    (MenuType::Window, "窗口(W)"),
                    (MenuType::Help, "帮助(H)"),
                ];

                for (i, (menu_type, label)) in menu_items.iter().enumerate() {
                    let response = ui.add(egui::Label::new(*label).sense(egui::Sense::click()));

                    if response.clicked() {
                        app.menu_click_pos = Some(response.rect.left_bottom());
                        app.menu_open_time = Some(ctx.input(|i| i.time) + 0.1);

                        if app.active_menu.as_ref() == Some(menu_type) {
                            app.active_menu = None;
                            app.menu_open_time = None;
                            app.menu_click_pos = None;
                        } else {
                            app.active_menu = Some(menu_type.clone());
                        }
                    }

                    if i < menu_items.len() - 1 {
                        ui.add_space(8.0);
                    }
                }
            });
        });
    }

    /// 渲染下拉菜单
    pub fn render_dropdown_menu(app: &mut MyApp, ctx: &egui::Context) {
        if let (Some(active_menu), Some(click_pos)) = (&app.active_menu, app.menu_click_pos) {
            if let Some(menu_items) = app.menu_data.get(active_menu) {
                let menu_height = (menu_items.len() as f32 * 25.0) + 10.0;

                let menu_items_clone = menu_items.clone();
                let active_menu_clone = active_menu.clone();
                
                egui::Window::new("")
                    .title_bar(false)
                    .resizable(false)
                    .fixed_pos(click_pos)
                    .fixed_size(egui::vec2(200.0, menu_height))
                    .show(ctx, |ui| {
                        let mut app_clone = app.clone();
                        Self::render_menu_items(ui, &mut app_clone, &menu_items_clone);
                        
                        // 同步状态回原应用
                        if app_clone.active_menu != Some(active_menu_clone) {
                            app.active_menu = app_clone.active_menu;
                            app.menu_open_time = app_clone.menu_open_time;
                            app.menu_click_pos = app_clone.menu_click_pos;
                        }
                    });
            }
        }
    }

    /// 渲染菜单项
    fn render_menu_items(ui: &mut egui::Ui, app: &mut MyApp, menu_items: &[crate::app::MenuItem]) {
        ui.vertical(|ui| {
            for item in menu_items {
                if item.label == "---" {
                    ui.separator();
                } else {
                    Self::render_menu_item(ui, app, item);
                }
            }
        });
    }

    /// 渲染单个菜单项
    fn render_menu_item(ui: &mut egui::Ui, app: &mut MyApp, item: &crate::app::MenuItem) {
        if item.enabled {
            let response = ui
                .horizontal(|ui| {
                    // 菜单项标签
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.label(&item.label);
                    });

                    // 快捷键
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if let Some(shortcut) = &item.shortcut {
                            ui.label(shortcut);
                        }
                    });
                })
                .response;

            if response.clicked() {
                app.handle_menu_action(&item.action);
                app.active_menu = None;
                app.menu_open_time = None;
                app.menu_click_pos = None;
            }
        } else {
            // 禁用的菜单项
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.label(egui::RichText::new(&item.label).color(egui::Color32::GRAY));
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if let Some(shortcut) = &item.shortcut {
                        ui.label(egui::RichText::new(shortcut).color(egui::Color32::GRAY));
                    }
                });
            });
        }
    }

    /// 渲染主内容区域
    pub fn render_main_content(ui: &mut egui::Ui, app: &mut MyApp) {
        ui.horizontal(|ui| {
            ui.label("您的姓名: ");
            ui.text_edit_singleline(&mut app.name);
        });

        ui.add(egui::Slider::new(&mut app.age, 0..=120).text("年龄"));

        if ui.button("每年点击").clicked() {
            app.age += 1;
        }

        ui.label(format!("你好 '{}', 年龄 {}", app.name, app.age));

        // 显示当前文件信息
        if let Some(file) = &app.current_file {
            ui.label(format!("当前文件: {}", file));
        }

        // 深色模式切换
        if ui.button("切换深色模式").clicked() {
            app.toggle_dark_mode();
        }
    }

    /// 渲染侧边栏
    pub fn render_sidebar(ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.label("项目导航");
            ui.separator();

            if ui.button("文件管理器").clicked() {
                println!("打开文件管理器");
            }

            if ui.button("数据库连接").clicked() {
                println!("打开数据库连接");
            }

            if ui.button("设置").clicked() {
                println!("打开设置");
            }
        });
    }

    /// 渲染底部状态栏
    pub fn render_status_bar(ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("就绪");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label("egui 应用");
            });
        });
    }
}
