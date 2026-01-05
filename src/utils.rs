//! 工具模块
//! 提供通用的工具函数

use std::fs;
use std::path::Path;

/// 文件操作工具
pub struct FileUtils;

impl FileUtils {
    /// 检查文件是否存在
    pub fn file_exists(path: &str) -> bool {
        Path::new(path).exists()
    }

    /// 读取文件内容
    pub fn read_file(path: &str) -> Result<String, String> {
        fs::read_to_string(path).map_err(|e| e.to_string())
    }

    /// 写入文件内容
    pub fn write_file(path: &str, content: &str) -> Result<(), String> {
        if let Some(parent) = Path::new(path).parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        fs::write(path, content).map_err(|e| e.to_string())
    }

    /// 获取文件扩展名
    pub fn get_extension(path: &str) -> Option<String> {
        Path::new(path)
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|s| s.to_lowercase())
    }

    /// 获取文件名（不含路径）
    pub fn get_filename(path: &str) -> Option<String> {
        Path::new(path)
            .file_name()
            .and_then(|name| name.to_str())
            .map(|s| s.to_string())
    }
}

/// 字符串工具
pub struct StringUtils;

impl StringUtils {
    /// 检查字符串是否为空或仅包含空白字符
    pub fn is_blank(s: &str) -> bool {
        s.trim().is_empty()
    }

    /// 截断字符串并添加省略号
    pub fn truncate_with_ellipsis(s: &str, max_len: usize) -> String {
        if s.len() <= max_len {
            return s.to_string();
        }

        if max_len <= 3 {
            return "...".to_string();
        }

        format!("{}...", &s[..max_len - 3])
    }

    /// 检查字符串是否为有效的文件名
    pub fn is_valid_filename(s: &str) -> bool {
        !s.is_empty()
            && !s.contains('/')
            && !s.contains('\\')
            && !s.contains(':')
            && !s.contains('*')
            && !s.contains('?')
            && !s.contains('"')
            && !s.contains('<')
            && !s.contains('>')
            && !s.contains('|')
    }
}

/// 时间工具
pub struct TimeUtils;

impl TimeUtils {
    /// 获取当前时间戳（秒）
    pub fn current_timestamp() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    /// 格式化时间为字符串
    pub fn format_time(timestamp: u64) -> String {
        let dt = chrono::DateTime::from_timestamp(timestamp as i64, 0)
            .unwrap_or_else(|| chrono::DateTime::from_timestamp(0, 0).unwrap());
        dt.format("%Y-%m-%d %H:%M:%S").to_string()
    }

    /// 获取当前格式化时间
    pub fn current_time_string() -> String {
        Self::format_time(Self::current_timestamp())
    }
}

/// 数学工具
pub struct MathUtils;

impl MathUtils {
    /// 将值限制在指定范围内
    pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
        if value < min {
            min
        } else if value > max {
            max
        } else {
            value
        }
    }

    /// 线性插值
    pub fn lerp(start: f32, end: f32, t: f32) -> f32 {
        start + (end - start) * t.clamp(0.0, 1.0)
    }

    /// 映射值到新范围
    pub fn map(value: f32, from_min: f32, from_max: f32, to_min: f32, to_max: f32) -> f32 {
        let normalized = (value - from_min) / (from_max - from_min);
        to_min + normalized * (to_max - to_min)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_truncate() {
        assert_eq!(StringUtils::truncate_with_ellipsis("hello", 5), "hello");
        assert_eq!(
            StringUtils::truncate_with_ellipsis("hello world", 8),
            "hello..."
        );
    }

    #[test]
    fn test_filename_validation() {
        assert!(StringUtils::is_valid_filename("test.txt"));
        assert!(!StringUtils::is_valid_filename("test/txt"));
        assert!(!StringUtils::is_valid_filename(""));
    }

    #[test]
    fn test_math_clamp() {
        assert_eq!(MathUtils::clamp(5, 0, 10), 5);
        assert_eq!(MathUtils::clamp(-1, 0, 10), 0);
        assert_eq!(MathUtils::clamp(15, 0, 10), 10);
    }
}
