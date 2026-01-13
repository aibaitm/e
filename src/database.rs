//! 数据库模块
//! 处理数据库连接和操作

use duckdb::{Connection, Result};
use std::io;

/// 自定义数据库错误类型
#[derive(Debug)]
pub enum DatabaseError {
    NotConnected,
    DuckDB(duckdb::Error),
    IO(io::Error),
}

impl From<duckdb::Error> for DatabaseError {
    fn from(err: duckdb::Error) -> Self {
        DatabaseError::DuckDB(err)
    }
}

impl From<io::Error> for DatabaseError {
    fn from(err: io::Error) -> Self {
        DatabaseError::IO(err)
    }
}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatabaseError::NotConnected => write!(f, "未连接到数据库"),
            DatabaseError::DuckDB(e) => write!(f, "数据库错误: {}", e),
            DatabaseError::IO(e) => write!(f, "IO错误: {}", e),
        }
    }
}

impl std::error::Error for DatabaseError {}

/// 数据库管理器
pub struct DatabaseManager {
    connection: Option<Connection>,
    current_db_path: Option<String>,
}

impl DatabaseManager {
    /// 创建新的数据库管理器
    pub fn new() -> Self {
        Self {
            connection: None,
            current_db_path: None,
        }
    }

    /// 连接到数据库
    pub fn connect(&mut self, db_path: &str) -> Result<()> {
        match Connection::open(db_path) {
            Ok(conn) => {
                self.connection = Some(conn);
                self.current_db_path = Some(db_path.to_string());
                println!("成功连接到数据库: {}", db_path);
                Ok(())
            }
            Err(e) => {
                eprintln!("连接数据库失败: {}", e);
                Err(e)
            }
        }
    }

    /// 断开数据库连接
    pub fn disconnect(&mut self) {
        self.connection = None;
        self.current_db_path = None;
        println!("已断开数据库连接");
    }

    /// 执行 SQL 查询
    pub fn execute_query(&self, query: &str) -> Result<(), DatabaseError> {
        if let Some(ref conn) = self.connection {
            conn.execute(query, [])?;
            println!("执行查询: {}", query);
            Ok(())
        } else {
            Err(DatabaseError::NotConnected)
        }
    }

    /// 检查是否已连接
    pub fn is_connected(&self) -> bool {
        self.connection.is_some()
    }

    /// 获取当前数据库路径
    pub fn current_db_path(&self) -> Option<&str> {
        self.current_db_path.as_deref()
    }

    /// 获取表列表
    pub fn get_tables(&self) -> Result<Vec<String>, DatabaseError> {
        if let Some(ref conn) = self.connection {
            let mut stmt =
                conn.prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")?;

            let tables = stmt
                .query_map([], |row| Ok(row.get::<_, String>(0)?))?
                .collect::<Result<Vec<String>>>()?;

            Ok(tables)
        } else {
            Err(DatabaseError::NotConnected)
        }
    }

    /// 执行查询并返回结果
    pub fn query(&self, sql: &str) -> Result<Vec<Vec<String>>, DatabaseError> {
        if let Some(ref conn) = self.connection {
            let mut stmt = conn.prepare(sql)?;
            let column_count = stmt.column_count();

            let rows = stmt
                .query_map([], |row| {
                    let mut result = Vec::with_capacity(column_count);
                    for i in 0..column_count {
                        let value: String = row.get(i)?;
                        result.push(value);
                    }
                    Ok(result)
                })?
                .collect::<Result<Vec<Vec<String>>>>()?;

            Ok(rows)
        } else {
            Err(DatabaseError::NotConnected)
        }
    }
}

impl Default for DatabaseManager {
    fn default() -> Self {
        Self::new()
    }
}
