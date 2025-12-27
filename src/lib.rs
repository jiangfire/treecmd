//! Treecmd 库 - 现代tree命令行工具
//!
//! 这个库提供了目录遍历、格式化和输出功能。

pub mod config;
pub mod walker;
pub mod formatter;

// 导出主要类型以便测试
pub use config::{Args, Config};
pub use walker::Walker;
pub use formatter::{Formatter, FileNode};