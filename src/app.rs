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
}

impl Default for MyApp {
    fn default() -> Self {
        let mut menu_data = HashMap::new();

        // 文件菜单
        menu_data.insert(
            MenuType::File,
            vec![
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
        }
    }
}

impl MyApp {
    /// 处理菜单项点击
    pub fn handle_menu_action(&mut self, action: &str) {
        match action {
            "new_file" => self.new_file(),
            "open_file" => self.open_file(),
            "save_file" => self.save_file(),
            "exit" => self.exit(),
            "toggle_dark_mode" => self.toggle_dark_mode(),
            _ => println!("执行动作: {}", action),
        }
    }

    /// 新建文件
    fn new_file(&mut self) {
        self.current_file = None;
        self.name = "新文件".to_owned();
        println!("创建新文件");
    }

    /// 打开文件
    fn open_file(&mut self) {
        // 这里会调用文件对话框
        println!("打开文件对话框");
    }

    /// 保存文件
    fn save_file(&mut self) {
        println!("保存文件");
    }

    /// 退出应用
    fn exit(&mut self) {
        println!("退出应用");
        // 在实际应用中，这里会触发应用关闭
    }

    /// 切换深色模式
    pub fn toggle_dark_mode(&mut self) {
        self.is_dark_mode = !self.is_dark_mode;
        println!("切换深色模式: {}", self.is_dark_mode);
    }
}
