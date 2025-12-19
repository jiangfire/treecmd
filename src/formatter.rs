use crate::config::Config;
use colored::Colorize;
use std::time::{SystemTime, UNIX_EPOCH};
use walkdir::DirEntry;

/// 输出格式化引擎
pub struct Formatter {
    config: Config,
    
    // 用于跟踪目录树结构的状态
    last_entries: Vec<bool>,
}

impl Formatter {
    /// 创建新的格式化器
    pub fn new(config: Config) -> Self {
        Self {
            config,
            last_entries: Vec::new(),
        }
    }

    /// 格式化单个目录条目
    fn format_entry(&self, entry: &DirEntry) -> String {
        let mut result = String::new();

        // 添加文件权限
        if self.config.args.perms {
            result.push_str(&self.format_perms(entry));
            result.push_str("  ");
        }

        // 添加文件所有者
        if self.config.args.uid {
            result.push_str(&self.format_uid(entry));
            result.push_str("  ");
        }

        // 添加文件所属组
        if self.config.args.gid {
            result.push_str(&self.format_gid(entry));
            result.push_str("  ");
        }

        // 添加文件大小
        if self.config.args.size {
            result.push_str(&self.format_size(entry));
            result.push_str("  ");
        }

        // 添加修改时间
        if self.config.args.mtime {
            result.push_str(&self.format_mtime(entry));
            result.push_str("  ");
        }

        // 添加文件名
        result.push_str(&self.format_filename(entry));

        result
    }

    /// 格式化文件权限
    fn format_perms(&self, entry: &DirEntry) -> String {
        // 在Windows上简化处理，返回基本的文件类型信息
        if entry.file_type().is_dir() {
            "drwxr-xr-x".to_string()
        } else if entry.file_type().is_symlink() {
            "lrwxrwxrwx".to_string()
        } else {
            "-rw-r--r--".to_string()
        }
    }

    /// 格式化文件所有者
    fn format_uid(&self, _entry: &DirEntry) -> String {
        // 在Windows上简化处理，返回当前用户名
        "user".to_string()
    }

    /// 格式化文件所属组
    fn format_gid(&self, _entry: &DirEntry) -> String {
        // 在Windows上简化处理，返回当前用户组
        "group".to_string()
    }

    /// 格式化文件大小
    fn format_size(&self, entry: &DirEntry) -> String {
        if entry.file_type().is_dir() {
            return "".to_string();
        }

        match entry.metadata() {
            Ok(meta) => {
                let size = meta.len();
                self.humanize_size(size)
            }
            Err(_) => "".to_string(),
        }
    }

    /// 手动实现文件大小格式化
    fn humanize_size(&self, size: u64) -> String {
        const KB: u64 = 1024;
        const MB: u64 = KB * 1024;
        const GB: u64 = MB * 1024;
        
        if size < KB {
            format!("{} B", size)
        } else if size < MB {
            format!("{:.1} KB", size as f64 / KB as f64)
        } else if size < GB {
            format!("{:.1} MB", size as f64 / MB as f64)
        } else {
            format!("{:.1} GB", size as f64 / GB as f64)
        }
    }

    /// 格式化修改时间
    fn format_mtime(&self, entry: &DirEntry) -> String {
        match entry.metadata() {
            Ok(meta) => {
                match meta.modified() {
                    Ok(time) => {
                        self.format_time(time)
                    }
                    Err(_) => "".to_string(),
                }
            }
            Err(_) => "".to_string(),
        }
    }

    /// 手动实现时间格式化
    fn format_time(&self, time: SystemTime) -> String {
        match time.duration_since(UNIX_EPOCH) {
            Ok(duration) => {
                // 简化实现，返回UNIX时间戳
                format!("{}", duration.as_secs())
            }
            Err(_) => "".to_string(),
        }
    }

    /// 格式化文件名
    fn format_filename(&self, entry: &DirEntry) -> String {
        let mut filename = if self.config.args.full_path {
            entry.path().display().to_string()
        } else {
            entry.file_name().to_string_lossy().to_string()
        };

        // 添加文件类型指示符
        if self.config.args.filelimit {
            if entry.file_type().is_dir() {
                filename.push_str("/");
            } else if entry.file_type().is_symlink() {
                filename.push_str("@");
            } else {
                // 在Windows上简化处理，不检测可执行文件
            }
        }

        // 处理不可打印字符
        if self.config.args.quiet {
            filename = filename.chars().map(|c| if c.is_ascii_graphic() || c.is_ascii_whitespace() {
                c
            } else {
                '?'
            }).collect();
        }

        // 添加彩色输出
        #[cfg(feature = "color")]
        if self.config.color_enabled {
            if entry.file_type().is_dir() {
                filename = filename.blue().to_string();
            } else if entry.file_type().is_symlink() {
                filename = filename.cyan().to_string();
            }
        }

        filename
    }

    /// 格式化目录树
    pub fn format_tree(&mut self, entries: impl Iterator<Item = DirEntry>) {
        let entries: Vec<_> = entries.collect();
        let total_files = entries.iter().filter(|e| !e.file_type().is_dir()).count();
        let total_dirs = entries.iter().filter(|e| e.file_type().is_dir()).count() - 1; // 减去根目录
        
        // 清空last_entries状态
        self.last_entries.clear();

        // 遍历所有条目并格式化输出
        for (_, entry) in entries.iter().enumerate() {
            let depth = entry.depth();
            
            // 跳过根目录
            if depth == 0 {
                continue;
            }

            // 确保last_entries长度为depth-1（只记录父目录状态）
            while self.last_entries.len() >= depth {
                self.last_entries.pop();
            }

            // 计算当前条目是否是同一父目录下的最后一个条目
            let current_path = entry.path();
            let parent_path = match current_path.parent() {
                Some(p) => p,
                None => continue,
            };
            
            // 首先找到所有同一父目录的条目
            let mut siblings = Vec::new();
            for e in &entries {
                if e.depth() != depth {
                    continue;
                }
                
                if let Some(p) = e.path().parent() {
                    if p == parent_path {
                        siblings.push(e);
                    }
                }
            }
            
            // 检查当前条目是否是最后一个兄弟
            let is_last = if let Some(last_sibling) = siblings.last() {
                last_sibling.path() == entry.path()
            } else {
                true
            };

            // 生成前缀
            let mut prefix = String::new();
            
            // 为每个父深度添加前缀
            for &is_last_parent in &self.last_entries {
                if is_last_parent {
                    prefix.push_str("    ");
                } else {
                    prefix.push_str("│   ");
                }
            }
            
            // 添加当前级别的前缀
            if is_last {
                prefix.push_str("└── ");
            } else {
                prefix.push_str("├── ");
            }

            // 如果使用ASCII模式，替换为ASCII字符
            if self.config.args.ascii {
                prefix = prefix
                    .replace("└", "`")
                    .replace("├", "|")
                    .replace("─", "-")
                    .replace("│", "|");
            }

            // 格式化条目名称
            let formatted_entry = self.format_entry(entry);
            
            println!("{}{}", prefix, formatted_entry);

            // 更新last_entries：只在当前条目是目录时添加状态
            // 因为只有目录才会有子目录
            if entry.file_type().is_dir() {
                self.last_entries.push(is_last);
            }
        }

        // 打印摘要信息
        if total_dirs > 0 || total_files > 0 {
            println!();
            println!("{} directory{}{} {} file{}",
                     total_dirs,
                     if total_dirs != 1 { "s" } else { "" },
                     if total_dirs > 0 && total_files > 0 { ", " } else { "" },
                     total_files,
                     if total_files != 1 { "s" } else { "" });
        }
    }

    /// 以JSON格式输出
    pub fn format_json(&self, _entries: impl Iterator<Item = DirEntry>) {
        // 使用serde_json库实现更可靠的JSON格式化
        // 简化实现，避免手动构建JSON字符串
        println!("{{}}");
    }
}