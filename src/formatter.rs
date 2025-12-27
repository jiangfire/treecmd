use crate::config::Config;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use walkdir::DirEntry;

/// JSON输出的文件结构
#[derive(Serialize, Deserialize, Debug)]
pub struct FileNode {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub size: u64,
    pub modified: u64,
    pub children: Vec<FileNode>,
}

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
            Ok(meta) => match meta.modified() {
                Ok(time) => self.format_time(time),
                Err(_) => "".to_string(),
            },
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

    /// 检查文件是否可执行
    fn is_executable(&self, entry: &DirEntry) -> bool {
        // 在Windows上，检查.exe扩展名
        #[cfg(target_os = "windows")]
        {
            if let Some(filename) = entry.file_name().to_str() {
                return filename.to_lowercase().ends_with(".exe");
            }
            false
        }

        // 在Linux/macOS上，检查执行权限
        #[cfg(not(target_os = "windows"))]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(metadata) = entry.metadata() {
                let permissions = metadata.permissions();
                // 检查是否有执行权限位（owner/group/others任意一个有执行权限）
                return (permissions.mode() & 0o111) != 0;
            }
            false
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
                filename.push('/');
            } else if entry.file_type().is_symlink() {
                filename.push('@');
            } else if self.is_executable(entry) {
                // 可执行文件添加 * 标记
                filename.push('*');
            }
        }

        // 处理不可打印字符
        if self.config.args.quiet {
            filename = filename
                .chars()
                .map(|c| {
                    if c.is_ascii_graphic() || c.is_ascii_whitespace() {
                        c
                    } else {
                        '?'
                    }
                })
                .collect();
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
        let total_dirs = entries.iter().filter(|e| e.file_type().is_dir()).count();
        let total_dirs_display = if total_dirs > 0 { total_dirs - 1 } else { 0 }; // 减去根目录

        // 如果启用noreport模式，只显示文件列表，不显示缩进线和摘要
        if self.config.args.noreport {
            for entry in entries.iter() {
                if entry.depth() == 0 {
                    continue; // 跳过根目录
                }
                // 只输出文件名，不带任何前缀
                let formatted_entry = self.format_entry(entry);
                println!("{}", formatted_entry);
            }
            return;
        }

        // 清空last_entries状态
        self.last_entries.clear();

        // 遍历所有条目并格式化输出
        for entry in entries.iter() {
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

                if let Some(p) = e.path().parent()
                    && p == parent_path
                {
                    siblings.push(e);
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
        if total_dirs_display > 0 || total_files > 0 {
            println!();
            println!(
                "{} directory{}{} {} file{}",
                total_dirs_display,
                if total_dirs_display != 1 { "s" } else { "" },
                if total_dirs_display > 0 && total_files > 0 {
                    ", "
                } else {
                    ""
                },
                total_files,
                if total_files != 1 { "s" } else { "" }
            );
        }
    }

    /// 构建文件节点树
    fn build_file_tree(&self, entries: &[DirEntry]) -> FileNode {
        // 找到深度为0的条目（根目录）
        let root_entries: Vec<&DirEntry> = entries.iter().filter(|e| e.depth() == 0).collect();

        if root_entries.is_empty() {
            // 如果没有根目录，返回空节点
            return FileNode {
                name: ".".to_string(),
                path: ".".to_string(),
                is_dir: true,
                size: 0,
                modified: 0,
                children: Vec::new(),
            };
        }

        // 使用第一个根目录作为根节点
        let root_entry = root_entries[0];
        let root_node = FileNode {
            name: root_entry.file_name().to_string_lossy().to_string(),
            path: root_entry.path().display().to_string(),
            is_dir: true,
            size: match root_entry.metadata() {
                Ok(meta) => meta.len(),
                Err(_) => 0,
            },
            modified: match root_entry.metadata() {
                Ok(meta) => match meta.modified() {
                    Ok(time) => match time.duration_since(UNIX_EPOCH) {
                        Ok(dur) => dur.as_secs(),
                        Err(_) => 0,
                    },
                    Err(_) => 0,
                },
                Err(_) => 0,
            },
            children: Vec::new(),
        };

        // 递归构建子节点
        self.build_children_recursive(root_node, entries, 1)
    }

    /// 递归构建子节点
    fn build_children_recursive(
        &self,
        mut parent: FileNode,
        entries: &[DirEntry],
        depth: usize,
    ) -> FileNode {
        // 找到当前深度的所有条目
        for entry in entries.iter().filter(|e| e.depth() == depth) {
            // 获取当前条目的父目录
            let child_path = entry.path();
            let child_parent = match child_path.parent() {
                Some(p) => p,
                None => continue,
            };

            // 检查当前条目的父目录是否与父节点的路径相同
            let parent_path = std::path::Path::new(&parent.path);
            if child_parent == parent_path {
                // 构建子节点
                let mut child_node = FileNode {
                    name: entry.file_name().to_string_lossy().to_string(),
                    path: entry.path().display().to_string(),
                    is_dir: entry.file_type().is_dir(),
                    size: match entry.metadata() {
                        Ok(meta) => meta.len(),
                        Err(_) => 0,
                    },
                    modified: match entry.metadata() {
                        Ok(meta) => match meta.modified() {
                            Ok(time) => match time.duration_since(UNIX_EPOCH) {
                                Ok(dur) => dur.as_secs(),
                                Err(_) => 0,
                            },
                            Err(_) => 0,
                        },
                        Err(_) => 0,
                    },
                    children: Vec::new(),
                };

                // 如果是目录，递归构建子节点
                if child_node.is_dir {
                    child_node = self.build_children_recursive(child_node, entries, depth + 1);
                }

                // 添加到父节点的子列表中
                parent.children.push(child_node);
            }
        }

        parent
    }

    /// 以JSON格式输出
    pub fn format_json(&self, entries: impl Iterator<Item = DirEntry>) {
        let entries: Vec<_> = entries.collect();

        // 构建文件树
        let file_tree = self.build_file_tree(&entries);

        // 序列化为JSON并输出
        match serde_json::to_string_pretty(&file_tree) {
            Ok(json) => println!("{}", json),
            Err(e) => eprintln!("Error generating JSON: {}", e),
        }
    }
}
