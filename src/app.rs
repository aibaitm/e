//! 主应用模块
//! 定义应用状态和核心逻辑

use eframe::egui;
use std::collections::HashMap;

/// 菜单类型枚举
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MenuType {
    File,
    Edit,
    Navigate,
    Search,
    SqlEditor,
    Database,
    Window,
    Help,
}

/// 菜单项结构
#[derive(Clone, PartialEq)]
pub struct MenuItem {
    pub label: String,
    pub shortcut: Option<String>,       // 快捷键
    pub action: String,                 // 动作描述
    pub enabled: bool,                  // 是否启用
    pub submenu: Option<Vec<MenuItem>>, // 子菜单
}

impl MenuItem {
    pub fn new(label: &str, shortcut: Option<&str>, action: &str) -> Self {
        Self {
            label: label.to_owned(),
            shortcut: shortcut.map(|s| s.to_owned()),
            action: action.to_owned(),
            enabled: true,
            submenu: None,
        }
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

/// 文件项
#[derive(Clone, Debug)]
pub struct FileItem {
    pub name: String,
    pub path: String,
    pub is_directory: bool,
    pub is_expanded: bool,
    pub children: Vec<FileItem>,
}

/// 文件资源管理器标签页
#[derive(Clone, Debug)]
pub struct ExplorerTab {
    pub name: String,
    pub path: String,
    pub file_tree: Vec<FileItem>,
    pub is_active: bool,
    pub expanded_paths: std::collections::HashSet<String>, // 记录展开的路径
}

/// 主应用状态
#[derive(Clone)]
pub struct MyApp {
    pub name: String,
    pub age: u32,
    pub active_menu: Option<MenuType>, // 当前打开的菜单
    pub menu_data: HashMap<MenuType, Vec<MenuItem>>, // 菜单项数据
    pub menu_open_time: Option<f64>,   // 菜单打开时的时间戳
    pub menu_click_pos: Option<egui::Pos2>, // 菜单点击位置
    pub current_file: Option<String>,  // 当前打开的文件
    pub is_dark_mode: bool,            // 深色模式
    pub status_message: String,        // 状态栏消息
    pub status_message_time: f64,      // 消息显示时间
    pub explorer_tabs: Vec<ExplorerTab>, // 文件资源管理器标签页
    pub active_explorer_tab: Option<usize>, // 当前活动的资源管理器标签页索引
}

impl Default for MyApp {
    fn default() -> Self {
        let mut menu_data = HashMap::new();

        // 文件菜单
        menu_data.insert(
            MenuType::File,
            vec![
                MenuItem::new("打开文件夹", Some("Ctrl+Shift+O"), "open_folder"),
                MenuItem::new("最近的编辑", None, "recent_edit"),
                MenuItem::new("查找指定文件...", Some("Ctrl+O"), "find_file"),
                MenuItem::new("新建", Some("Ctrl+N"), "new_file"),
                MenuItem::new("保存", Some("Ctrl+S"), "save_file"),
                MenuItem::new("另存为", None, "save_as"),
                MenuItem::new("保存全部", Some("Ctrl+Shift+S"), "save_all"),
                MenuItem::new("关闭", Some("Ctrl+W"), "close_file"),
                MenuItem::new("打印", Some("Ctrl+P"), "print_file"),
                MenuItem::new("重命名", Some("F2"), "rename_file"),
                MenuItem::new("刷新", Some("F5"), "refresh_file"),
                MenuItem::new("---", None, "separator"),
                MenuItem::new("导入", None, "import_file"),
                MenuItem::new("导出", None, "export_file"),
                MenuItem::new("---", None, "separator"),
                MenuItem::new("属性", None, "file_property"),
                MenuItem::new("---", None, "separator"),
                MenuItem::new("退出", Some("Ctrl+Q"), "exit"),
            ],
        );

        // 其他菜单项保持不变...
        menu_data.insert(
            MenuType::Edit,
            vec![
                MenuItem::new("撤销", Some("Ctrl+Z"), "undo"),
                MenuItem::new("重做", Some("Ctrl+Y"), "redo"),
                MenuItem::new("剪切", Some("Ctrl+X"), "cut"),
                MenuItem::new("复制", Some("Ctrl+C"), "copy"),
                MenuItem::new("粘贴", Some("Ctrl+V"), "paste"),
                MenuItem::new("删除", Some("Del"), "delete").enabled(false),
                MenuItem::new("---", None, "separator"),
                MenuItem::new("生成UUID", Some("Del"), "generate_uuid"),
            ],
        );

        menu_data.insert(
            MenuType::Navigate,
            vec![MenuItem::new("跳转到行", Some("Ctrl+G"), "goto_line")],
        );

        menu_data.insert(
            MenuType::Search,
            vec![MenuItem::new("查找", Some("Ctrl+F"), "find")],
        );

        menu_data.insert(
            MenuType::SqlEditor,
            vec![MenuItem::new("新建查询", None, "new_query")],
        );

        menu_data.insert(
            MenuType::Database,
            vec![MenuItem::new("连接数据库", None, "connect_db")],
        );

        menu_data.insert(
            MenuType::Window,
            vec![MenuItem::new(
                "新建窗口",
                Some("Ctrl+Shift+N"),
                "new_window",
            )],
        );

        menu_data.insert(MenuType::Help, vec![MenuItem::new("关于", None, "about")]);

        Self {
            name: "Arthur".to_owned(),
            age: 42,
            active_menu: None,
            menu_data,
            menu_open_time: None,
            menu_click_pos: None,
            current_file: None,
            is_dark_mode: false,
            status_message: "就绪".to_owned(),
            status_message_time: 0.0,
            explorer_tabs: Vec::new(),
            active_explorer_tab: None,
        }
    }
}

impl MyApp {
    /// 处理菜单项点击
    pub fn handle_menu_action(&mut self, action: &str) {
        println!("handle_menu_action 被调用，动作: {}", action);
        
        match action {
            "open_folder" => self.open_folder(),
            "new_file" => self.new_file(),
            "open_file" => self.open_file(),
            "save_file" => self.save_file(),
            "save_as" => self.save_as(),
            "save_all" => self.save_all(),
            "close_file" => self.close_file(),
            "print_file" => self.print_file(),
            "rename_file" => self.rename_file(),
            "refresh_file" => self.refresh_file(),
            "import_file" => self.import_file(),
            "export_file" => self.export_file(),
            "file_property" => self.show_file_property(),
            "undo" => self.undo(),
            "redo" => self.redo(),
            "cut" => self.cut(),
            "copy" => self.copy(),
            "paste" => self.paste(),
            "delete" => self.delete(),
            "generate_uuid" => self.generate_uuid(),
            "goto_line" => self.goto_line(),
            "find" => self.find(),
            "new_query" => self.new_query(),
            "connect_db" => self.connect_database(),
            "new_window" => self.new_window(),
            "about" => self.show_about(),
            "exit" => self.exit(),
            "toggle_dark_mode" => self.toggle_dark_mode(),
            _ => println!("执行动作: {}", action),
        }
    }

    /// 打开文件夹
    fn open_folder(&mut self) {
        self.set_status_message("正在打开文件夹...".to_owned());

        // 使用Windows文件选择对话框
        let folder_path = match rfd::FileDialog::new().set_title("选择文件夹").pick_folder() {
            Some(path) => path.to_string_lossy().to_string(),
            None => {
                self.set_status_message("用户取消了文件夹选择".to_owned());
                return;
            }
        };

        // 创建新的资源管理器标签页
        self.add_explorer_tab(&folder_path);

        println!("打开文件夹: {}", folder_path);
    }

    /// 添加文件资源管理器标签页
    fn add_explorer_tab(&mut self, folder_path: &str) {
        let folder_name = match std::path::Path::new(folder_path).file_name() {
            Some(name) => name.to_string_lossy().to_string(),
            None => "文件夹".to_string(),
        };

        // 创建新的资源管理器标签页
        let new_tab = ExplorerTab {
            name: "文件资源管理器".to_string(), // 固定标签名
            path: folder_path.to_string(),
            file_tree: Vec::new(),
            is_active: true,
            expanded_paths: std::collections::HashSet::new(),
        };

        // 加载文件夹内容
        let file_tree = self.load_folder_tree(folder_path);
        let mut new_tab = new_tab;
        new_tab.file_tree = file_tree;

        // 添加新标签页并设置为活动状态
        let new_index = self.explorer_tabs.len();
        self.explorer_tabs.push(new_tab);
        self.active_explorer_tab = Some(new_index);

        // 更新其他标签页为非活动状态
        for i in 0..new_index {
            if let Some(tab) = self.explorer_tabs.get_mut(i) {
                tab.is_active = false;
            }
        }

        self.set_status_message(format!("已打开文件夹: {}", folder_name));
    }

    /// 加载文件夹树状结构
    fn load_folder_tree(&self, folder_path: &str) -> Vec<FileItem> {
        let mut file_tree = Vec::new();

        // 读取真实的文件系统目录结构
        if let Ok(entries) = std::fs::read_dir(folder_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    let is_directory = path.is_dir();

                    // 获取文件名
                    let name = match path.file_name() {
                        Some(os_str) => os_str.to_string_lossy().to_string(),
                        None => continue,
                    };

                    // 跳过隐藏文件（以 . 开头的文件/文件夹）
                    if name.starts_with('.') {
                        continue;
                    }

                    // 创建文件项
                    let mut file_item = FileItem {
                        name,
                        path: path.to_string_lossy().to_string(),
                        is_directory,
                        is_expanded: false,
                        children: Vec::new(),
                    };

                    // 如果是目录，递归加载子项（但不展开，节省性能）
                    if is_directory {
                        // 这里可以添加一个简单的子项来标记是否有内容
                        // 实际展开时再加载详细内容
                        file_item.children.push(FileItem {
                            name: "...".to_string(),
                            path: "".to_string(),
                            is_directory: false,
                            is_expanded: false,
                            children: Vec::new(),
                        });
                    }

                    file_tree.push(file_item);
                }
            }
        }

        // 按文件夹在前，文件在后，然后按名称排序
        file_tree.sort_by(|a, b| {
            if a.is_directory != b.is_directory {
                b.is_directory.cmp(&a.is_directory) // 文件夹在前
            } else {
                a.name.to_lowercase().cmp(&b.name.to_lowercase())
            }
        });

        file_tree
    }

    /// 新建文件
    fn new_file(&mut self) {
        self.current_file = None;
        self.name = "新文件".to_owned();
        self.set_status_message("已创建新文件".to_owned());
        println!("创建新文件");
    }

    /// 打开文件
    fn open_file(&mut self) {
        // 这里会调用文件对话框
        self.set_status_message("正在打开文件对话框...".to_owned());
        println!("打开文件对话框");
    }

    /// 保存文件
    fn save_file(&mut self) {
        self.set_status_message("正在保存文件...".to_owned());
        println!("保存文件");
    }

    /// 退出应用
    fn exit(&mut self) {
        self.set_status_message("正在退出应用...".to_owned());
        println!("退出应用");
        // 在实际应用中，这里会触发应用关闭
    }

    /// 切换深色模式
    pub fn toggle_dark_mode(&mut self) {
        self.is_dark_mode = !self.is_dark_mode;
        let mode = if self.is_dark_mode {
            "深色"
        } else {
            "浅色"
        };
        self.set_status_message(format!("已切换到{}模式", mode));
        println!("切换深色模式: {}", self.is_dark_mode);
    }

    /// 另存为
    fn save_as(&mut self) {
        self.set_status_message("正在打开另存为对话框...".to_owned());
        println!("另存为文件");
    }

    /// 保存全部
    fn save_all(&mut self) {
        self.set_status_message("正在保存所有打开的文件...".to_owned());
        println!("保存全部文件");
    }

    /// 关闭文件
    fn close_file(&mut self) {
        if self.current_file.is_some() {
            self.set_status_message("正在关闭当前文件...".to_owned());
            self.current_file = None;
        } else {
            self.set_status_message("没有打开的文件".to_owned());
        }
        println!("关闭文件");
    }

    /// 打印文件
    fn print_file(&mut self) {
        self.set_status_message("正在准备打印...".to_owned());
        println!("打印文件");
    }

    /// 重命名文件
    fn rename_file(&mut self) {
        self.set_status_message("正在重命名文件...".to_owned());
        println!("重命名文件");
    }

    /// 刷新文件
    fn refresh_file(&mut self) {
        self.set_status_message("正在刷新文件内容...".to_owned());
        println!("刷新文件");
    }

    /// 导入文件
    fn import_file(&mut self) {
        self.set_status_message("正在导入文件...".to_owned());
        println!("导入文件");
    }

    /// 导出文件
    fn export_file(&mut self) {
        self.set_status_message("正在导出文件...".to_owned());
        println!("导出文件");
    }

    /// 显示文件属性
    fn show_file_property(&mut self) {
        self.set_status_message("正在显示文件属性...".to_owned());
        println!("显示文件属性");
    }

    /// 撤销操作
    fn undo(&mut self) {
        self.set_status_message("正在撤销上一步操作...".to_owned());
        println!("撤销操作");
    }

    /// 重做操作
    fn redo(&mut self) {
        self.set_status_message("正在重做下一步操作...".to_owned());
        println!("重做操作");
    }

    /// 剪切
    fn cut(&mut self) {
        self.set_status_message("正在剪切选中内容...".to_owned());
        println!("剪切操作");
    }

    /// 复制
    fn copy(&mut self) {
        self.set_status_message("正在复制选中内容...".to_owned());
        println!("复制操作");
    }

    /// 粘贴
    fn paste(&mut self) {
        self.set_status_message("正在粘贴内容...".to_owned());
        println!("粘贴操作");
    }

    /// 删除
    fn delete(&mut self) {
        self.set_status_message("正在删除选中内容...".to_owned());
        println!("删除操作");
    }

    /// 生成UUID
    fn generate_uuid(&mut self) {
        self.set_status_message("正在生成UUID...".to_owned());
        println!("生成UUID");
    }

    /// 跳转到行
    fn goto_line(&mut self) {
        self.set_status_message("正在打开跳转到行对话框...".to_owned());
        println!("跳转到行");
    }

    /// 查找
    fn find(&mut self) {
        self.set_status_message("正在打开查找对话框...".to_owned());
        println!("查找操作");
    }

    /// 新建查询
    fn new_query(&mut self) {
        self.set_status_message("正在创建新的SQL查询...".to_owned());
        println!("新建SQL查询");
    }

    /// 连接数据库
    fn connect_database(&mut self) {
        self.set_status_message("正在打开数据库连接对话框...".to_owned());
        println!("连接数据库");
    }

    /// 新建窗口
    fn new_window(&mut self) {
        self.set_status_message("正在创建新窗口...".to_owned());
        println!("新建窗口");
    }

    /// 显示关于信息
    fn show_about(&mut self) {
        self.set_status_message("正在显示关于信息...".to_owned());
        println!("显示关于信息");
    }

    /// 设置状态栏消息
    pub fn set_status_message(&mut self, message: String) {
        self.status_message = message;
        self.status_message_time = 0.0;
    }

    /// 更新状态消息（随时间清除）
    pub fn update_status_message(&mut self, ctx: &eframe::egui::Context) {
        if self.status_message_time > 0.0 && ctx.input(|i| i.time) - self.status_message_time > 3.0
        {
            // 3秒后恢复默认消息
            if self.status_message != "就绪" {
                self.status_message = "就绪".to_owned();
                self.status_message_time = 0.0;
            }
        }
    }
}
