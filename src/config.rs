//! 配置模块
//! 处理应用配置和设置

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// 应用配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub window_width: f32,
    pub window_height: f32,
    pub is_maximized: bool,
    pub is_dark_mode: bool,
    pub recent_files: Vec<String>,
    pub font_size: f32,
    pub language: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            window_width: 1200.0,
            window_height: 800.0,
            is_maximized: false,
            is_dark_mode: false,
            recent_files: Vec::new(),
            font_size: 14.0,
            language: "zh-CN".to_string(),
        }
    }
}

impl AppConfig {
    /// 从文件加载配置
    pub fn load() -> Self {
        let config_path = "config/app_config.json";

        if Path::new(config_path).exists() {
            match fs::read_to_string(config_path) {
                Ok(content) => match serde_json::from_str(&content) {
                    Ok(config) => {
                        println!("配置加载成功");
                        return config;
                    }
                    Err(e) => {
                        eprintln!("配置文件解析错误: {}", e);
                    }
                },
                Err(e) => {
                    eprintln!("读取配置文件失败: {}", e);
                }
            }
        }

        // 如果加载失败，返回默认配置
        println!("使用默认配置");
        Self::default()
    }

    /// 保存配置到文件
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_dir = "config";
        if !Path::new(config_dir).exists() {
            fs::create_dir_all(config_dir)?;
        }

        let config_path = format!("{}/app_config.json", config_dir);
        let content = serde_json::to_string_pretty(self)?;
        fs::write(config_path, content)?;

        println!("配置保存成功");
        Ok(())
    }

    /// 添加最近文件
    pub fn add_recent_file(&mut self, file_path: &str) {
        // 移除已存在的相同路径
        self.recent_files.retain(|f| f != file_path);

        // 添加到开头
        self.recent_files.insert(0, file_path.to_string());

        // 限制最近文件数量
        if self.recent_files.len() > 10 {
            self.recent_files.truncate(10);
        }
    }

    /// 获取窗口尺寸
    pub fn window_size(&self) -> (f32, f32) {
        (self.window_width, self.window_height)
    }

    /// 设置窗口尺寸
    pub fn set_window_size(&mut self, width: f32, height: f32) {
        self.window_width = width;
        self.window_height = height;
    }
}
