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
                    // 菜单项标签 - 添加点击感应
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.add(egui::Label::new(&item.label).sense(egui::Sense::click()));
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
    pub fn render_sidebar(ui: &mut egui::Ui, app: &mut MyApp) {
        ui.vertical(|ui| {
            // 标签页标签区域
            ui.horizontal(|ui| {
                // 可以在这里添加更多标签页标签
                if ui.button("文件管理器").clicked() {
                    app.handle_menu_action("open_folder");
                }

                if ui.button("数据库").clicked() {
                    app.set_status_message("正在打开数据库连接管理器...".to_owned());
                    println!("打开数据库连接");
                }

                if ui.button("设置").clicked() {
                    app.set_status_message("正在打开设置面板...".to_owned());
                    println!("打开设置");
                }
            });
            ui.separator();

            // 标签页内容区域
            ui.vertical(|ui| {
                // 文件资源管理器标签页内容
                if !app.explorer_tabs.is_empty() {
                    Self::render_explorer_tabs_content(ui, app);
                }
            });
        });
    }

    /// 渲染文件资源管理器标签页内容
    fn render_explorer_tabs_content(ui: &mut egui::Ui, app: &mut MyApp) {
        // 渲染当前活动标签页的内容
        if let Some(active_index) = app.active_explorer_tab {
            if let Some(tab) = app.explorer_tabs.get_mut(active_index) {
                ui.label(egui::RichText::new(&format!("{} - {}", tab.name, tab.path)).strong());

                // 保存状态消息到队列，稍后处理
                let mut status_messages = Vec::new();

                // 渲染文件树
                Self::render_file_tree(
                    ui,
                    &mut tab.file_tree,
                    &mut tab.expanded_paths,
                    |message| {
                        status_messages.push(message);
                    },
                );

                // 处理状态消息（在闭包外）
                for message in status_messages {
                    app.set_status_message(message);
                }
            }
        }
    }

    /// 渲染文件资源管理器标签页
    fn render_explorer_tabs(ui: &mut egui::Ui, app: &mut MyApp) {
        ui.horizontal(|ui| {
            ui.label("资源管理器:");

            // 显示所有标签页
            let tab_indices: Vec<usize> = (0..app.explorer_tabs.len()).collect();

            for i in tab_indices {
                let tab_name = app.explorer_tabs[i].name.clone();
                let is_active = app.explorer_tabs[i].is_active;

                let label = if is_active {
                    egui::RichText::new(&tab_name).strong()
                } else {
                    egui::RichText::new(&tab_name)
                };

                if ui.button(label).clicked() {
                    // 切换活动标签页
                    app.active_explorer_tab = Some(i);
                    for j in 0..app.explorer_tabs.len() {
                        if let Some(tab) = app.explorer_tabs.get_mut(j) {
                            tab.is_active = j == i;
                        }
                    }
                    app.set_status_message(format!("切换到资源管理器: {}", tab_name));
                }
            }
        });
    }

    /// 渲染文件树
    fn render_file_tree(
        ui: &mut egui::Ui,
        file_tree: &mut Vec<crate::app::FileItem>,
        expanded_paths: &mut std::collections::HashSet<String>,
        mut set_status_message: impl FnMut(String),
    ) {
        ui.vertical(|ui| {
            for item in file_tree.iter_mut() {
                Self::render_file_item(ui, item, expanded_paths, 0, &mut set_status_message);
            }
        });
    }

    /// 渲染单个文件项
    fn render_file_item(
        ui: &mut egui::Ui,
        item: &mut crate::app::FileItem,
        expanded_paths: &mut std::collections::HashSet<String>,
        depth: usize,
        set_status_message: &mut impl FnMut(String),
    ) {
        let indent = 16.0 * depth as f32;
        let is_expanded = expanded_paths.contains(&item.path);

        ui.horizontal(|ui| {
            ui.add_space(indent);

            // 文件夹图标和展开/折叠按钮
            if item.is_directory {
                let icon = if is_expanded { "📂" } else { "📁" };

                if ui.small_button(icon).clicked() {
                    // 切换展开状态
                    if is_expanded {
                        expanded_paths.remove(&item.path);
                        set_status_message(format!("折叠 {}", item.name));
                    } else {
                        expanded_paths.insert(item.path.clone());
                        set_status_message(format!("展开 {}", item.name));

                        // 如果文件夹还没有加载子项，现在加载
                        if item.children.len() == 1 && item.children[0].name == "..." {
                            if let Ok(entries) = std::fs::read_dir(&item.path) {
                                let mut children = Vec::new();
                                for entry in entries {
                                    if let Ok(entry) = entry {
                                        let path = entry.path();
                                        let is_directory = path.is_dir();

                                        // 获取文件名
                                        let name = match path.file_name() {
                                            Some(os_str) => os_str.to_string_lossy().to_string(),
                                            None => continue,
                                        };

                                        // 跳过隐藏文件
                                        if name.starts_with('.') {
                                            continue;
                                        }

                                        children.push(crate::app::FileItem {
                                            name,
                                            path: path.to_string_lossy().to_string(),
                                            is_directory,
                                            is_expanded: false,
                                            children: if is_directory {
                                                vec![crate::app::FileItem {
                                                    name: "...".to_string(),
                                                    path: "".to_string(),
                                                    is_directory: false,
                                                    is_expanded: false,
                                                    children: Vec::new(),
                                                }]
                                            } else {
                                                Vec::new()
                                            },
                                        });
                                    }
                                }

                                // 按文件夹在前，文件在后排序
                                children.sort_by(|a, b| {
                                    if a.is_directory != b.is_directory {
                                        b.is_directory.cmp(&a.is_directory)
                                    } else {
                                        a.name.to_lowercase().cmp(&b.name.to_lowercase())
                                    }
                                });

                                item.children = children;
                            }
                        }
                    }
                }
            } else {
                // 文件图标
                ui.label("📄");
            }

            // 文件/文件夹名称
            let response = ui.selectable_label(false, &item.name);

            if response.clicked() {
                if item.is_directory {
                    set_status_message(format!("打开文件夹: {}", item.name));
                } else {
                    set_status_message(format!("打开文件: {}", item.name));
                    // 注意：这里无法直接设置 current_file，需要其他方式处理
                }
            }
        });

        // 渲染子项（如果文件夹是展开的）
        if item.is_directory && is_expanded {
            for child in &mut item.children {
                Self::render_file_item(ui, child, expanded_paths, depth + 1, set_status_message);
            }
        }
    }

    /// 渲染底部状态栏
    pub fn render_status_bar(ui: &mut egui::Ui, app: &mut MyApp) {
        ui.horizontal(|ui| {
            // 显示当前状态消息
            ui.label(&app.status_message);

            // 右侧显示应用信息
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label("egui 应用");
            });
        });
    }
}
