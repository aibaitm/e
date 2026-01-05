//! 主应用库模块
//! 定义应用的核心结构和功能

pub mod app;
pub mod config;
pub mod database;
pub mod ui;
pub mod utils;

// 重新导出主要类型
pub use app::{MenuItem, MenuType, MyApp};
pub use config::AppConfig;
pub use database::DatabaseManager;
