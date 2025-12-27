use crate::config::Config;
use rayon::prelude::*;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};
use indicatif::{ProgressBar, ProgressStyle};

/// 目录遍历引擎
pub struct Walker {
    config: Config,
}

impl Walker {
    /// 创建新的遍历器
    pub fn new(config: Config) -> Self {
        Self {
            config,
        }
    }

    /// 获取起始路径
    fn get_start_path(&self) -> &Path {
        self.config
            .args
            .path
            .as_deref()
            .unwrap_or_else(|| Path::new("."))
    }

    /// 过滤目录条目
    fn filter_entry(&self, entry: &DirEntry) -> bool {
        // 根目录总是保留
        if entry.depth() == 0 {
            return true;
        }

        // 检查是否是隐藏文件/目录或隐藏目录的子目录
        if !self.config.args.all {
            // 检查当前条目是否是隐藏文件/目录
            if let Some(file_name) = entry.file_name().to_str() {
                if file_name.starts_with(".") {
                    return false;
                }
            }

            // 检查当前条目是否是隐藏目录的子目录
            let mut current_path = entry.path();
            while let Some(parent) = current_path.parent() {
                if let Some(file_name) = parent.file_name() {
                    if let Some(name_str) = file_name.to_str() {
                        if name_str.starts_with(".") {
                            return false;
                        }
                    }
                }
                current_path = parent;
            }
        }

        // 检查是否仅显示目录
        if self.config.args.dirs_only && !entry.file_type().is_dir() {
            return false;
        }

        // 检查排除模式
        if let Some(exclude_regex) = &self.config.exclude_regex {
            if let Some(file_name) = entry.file_name().to_str() {
                if exclude_regex.is_match(file_name) {
                    return false;
                }
            }
        }

        // 检查包含模式
        if let Some(include_regex) = &self.config.include_regex {
            if let Some(file_name) = entry.file_name().to_str() {
                if !include_regex.is_match(file_name) {
                    return false;
                }
            } else {
                // 如果文件名无法转换为字符串，且有包含模式，则排除
                return false;
            }
        }

        true
    }

    /// 遍历目录并返回符合条件的条目
    /// 如果启用进度显示，会返回带有进度条的迭代器
    pub fn walk(&self) -> Box<dyn Iterator<Item = DirEntry> + '_> {
        let start_path = self.get_start_path();
        let mut walkdir = WalkDir::new(start_path);

        // 设置遍历选项
        walkdir = walkdir.max_depth(self.config.depth);

        if self.config.args.follow_links {
            walkdir = walkdir.follow_links(true);
        }

        if self.config.args.samefilesystem {
            walkdir = walkdir.same_file_system(true);
        }

        // 执行遍历并过滤
        let iter = walkdir
            .into_iter()
            .filter_map(Result::ok)
            .filter(|entry| self.filter_entry(entry));

        // 如果启用进度显示，包装进度条
        if self.config.args.progress {
            let pb = ProgressBar::new_spinner();
            pb.set_style(
                ProgressStyle::default_spinner()
                    .template("{spinner:.green} {msg} [{elapsed_precise}] {pos} items")
                    .unwrap()
                    .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
            );
            pb.set_message("遍历目录中...");

            Box::new(iter.inspect(move |_| {
                pb.inc(1);
            }))
        } else {
            Box::new(iter)
        }
    }

    /// 对条目进行排序，保持深度优先顺序，只对同一目录下的条目排序
    pub fn sort_entries(&self, entries: Vec<DirEntry>) -> Vec<DirEntry> {
        // 如果没有条目，直接返回空列表
        if entries.is_empty() {
            return Vec::new();
        }

        let mut result = Vec::new();

        // 按目录分组
        let mut dir_groups = std::collections::HashMap::new();

        // 先处理根目录
        let mut root_found = false;

        for entry in &entries {
            if entry.depth() == 0 {
                result.push(entry.clone());
                root_found = true;
                continue;
            }

            // 找到父目录
            let parent_path = match entry.path().parent() {
                Some(p) => p,
                None => continue,
            };

            // 添加到父目录的组中
            dir_groups.entry(parent_path.to_path_buf())
                .or_insert_with(Vec::new)
                .push(entry.clone());
        }

        // 如果没有找到根目录，直接返回所有条目
        if !root_found {
            return entries;
        }

        // 递归处理子目录
        let root_dir = result[0].clone();
        self.process_dir_groups(&dir_groups, &root_dir, &mut result);

        result
    }

    /// 递归处理目录组
    fn process_dir_groups(
        &self,
        dir_groups: &std::collections::HashMap<std::path::PathBuf, Vec<DirEntry>>,
        dir: &DirEntry,
        result: &mut Vec<DirEntry>
    ) {
        // 检查当前目录是否有子目录
        if let Some(children) = dir_groups.get(dir.path()) {
            // 复制子目录列表并排序
            let mut sorted_children = children.clone();

            // 对当前目录的子条目进行排序
            sorted_children.sort_by(|a, b| {
                // 目录排在文件前面
                let a_is_dir = a.file_type().is_dir();
                let b_is_dir = b.file_type().is_dir();

                if a_is_dir != b_is_dir {
                    return if a_is_dir {
                        std::cmp::Ordering::Less
                    } else {
                        std::cmp::Ordering::Greater
                    };
                }

                // 对于同一类型的条目，按指定顺序排序
                match self.config.args.sort.as_deref() {
                    Some("time") => {
                        // 按修改时间排序，最新的在前
                        let a_meta = a.metadata().unwrap();
                        let b_meta = b.metadata().unwrap();
                        b_meta.modified().unwrap().cmp(&a_meta.modified().unwrap())
                    }
                    Some(_) | None => {
                        // 按名称排序（默认）
                        a.file_name().cmp(b.file_name())
                    }
                }
            });

            // 添加排序后的子条目到结果中
            for child in sorted_children {
                result.push(child.clone());

                // 如果是目录，递归处理
                if child.file_type().is_dir() {
                    self.process_dir_groups(dir_groups, &child, result);
                }
            }
        }
    }

    /// 并行遍历目录并返回排序后的条目
    /// 使用并行处理来加速大目录的遍历和排序
    pub fn walk_parallel(&self) -> Vec<DirEntry> {
        let start_path = self.get_start_path();

        // 创建进度条（如果启用进度显示）
        let pb = if self.config.args.progress {
            let pb = ProgressBar::new_spinner();
            pb.set_style(
                ProgressStyle::default_spinner()
                    .template("{spinner:.green} {msg} [{elapsed_precise}] {pos} items")
                    .unwrap()
                    .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
            );
            pb.set_message("遍历目录中...");
            Some(pb)
        } else {
            None
        };

        // 第一步：收集所有条目（串行，因为walkdir本身不是并行的）
        let all_entries: Vec<DirEntry> = WalkDir::new(start_path)
            .max_depth(self.config.depth)
            .follow_links(self.config.args.follow_links)
            .same_file_system(self.config.args.samefilesystem)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|entry| self.filter_entry(entry))
            .inspect(|_| {
                // 更新进度条
                if let Some(ref pb) = pb {
                    pb.inc(1);
                }
            })
            .collect();

        // 完成进度条
        if let Some(ref pb) = pb {
            pb.finish_with_message(format!("完成，共 {} 个条目", all_entries.len()));
        }

        if all_entries.is_empty() {
            return Vec::new();
        }

        // 第二步：使用并行处理进行排序
        self.parallel_sort_entries(all_entries)
    }

    /// 并行排序条目 - 使用rayon加速排序过程
    fn parallel_sort_entries(&self, entries: Vec<DirEntry>) -> Vec<DirEntry> {
        if entries.is_empty() {
            return Vec::new();
        }

        // 找到根目录
        let root_opt = entries.iter().find(|e| e.depth() == 0);
        if root_opt.is_none() {
            return entries;
        }
        let root = root_opt.unwrap().clone();

        // 按父目录分组
        let mut dir_groups: std::collections::HashMap<PathBuf, Vec<DirEntry>> = std::collections::HashMap::new();

        for entry in &entries {
            if entry.depth() == 0 {
                continue;
            }

            if let Some(parent_path) = entry.path().parent() {
                dir_groups.entry(parent_path.to_path_buf())
                    .or_insert_with(Vec::new)
                    .push(entry.clone());
            }
        }

        // 递归构建结果（使用并行处理每个目录的排序）
        let mut result = vec![root.clone()];
        self.parallel_process_dir_groups(&dir_groups, &root, &mut result);

        result
    }

    /// 并行处理目录组 - 对每个目录的子条目使用并行排序
    fn parallel_process_dir_groups(
        &self,
        dir_groups: &std::collections::HashMap<PathBuf, Vec<DirEntry>>,
        dir: &DirEntry,
        result: &mut Vec<DirEntry>,
    ) {
        if let Some(children) = dir_groups.get(dir.path()) {
            // 使用rayon并行排序当前目录的子条目
            let mut sorted_children: Vec<DirEntry> = children
                .par_iter()
                .map(|e| e.clone())
                .collect();

            sorted_children.par_sort_by(|a, b| {
                // 目录排在文件前面
                let a_is_dir = a.file_type().is_dir();
                let b_is_dir = b.file_type().is_dir();

                if a_is_dir != b_is_dir {
                    return if a_is_dir {
                        std::cmp::Ordering::Less
                    } else {
                        std::cmp::Ordering::Greater
                    };
                }

                // 对于同一类型的条目，按指定顺序排序
                match self.config.args.sort.as_deref() {
                    Some("time") => {
                        let a_meta = a.metadata().unwrap();
                        let b_meta = b.metadata().unwrap();
                        b_meta.modified().unwrap().cmp(&a_meta.modified().unwrap())
                    }
                    Some(_) | None => {
                        a.file_name().cmp(b.file_name())
                    }
                }
            });

            // 添加排序后的子条目
            for child in sorted_children {
                result.push(child.clone());

                if child.file_type().is_dir() {
                    self.parallel_process_dir_groups(dir_groups, &child, result);
                }
            }
        }
    }

}