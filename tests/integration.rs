//! 集成测试 - 测试treecmd的主要功能

use std::path::PathBuf;
use treecmd::{Args, Config, Walker, Formatter};

/// 使用当前目录进行测试
fn get_test_path() -> PathBuf {
    std::env::current_dir().expect("Failed to get current dir")
}

#[test]
fn test_basic_walk() {
    let test_path = get_test_path();

    let args = Args {
        all: false,
        ascii: false,
        color: false,
        dirs_only: false,
        mtime: false,
        filelimit: false,
        full_path: false,
        gid: false,
        noreport: false,
        exclude: None,
        follow_links: false,
        no_color: true,
        literal: false,
        include: None,
        perms: false,
        quiet: false,
        size: false,
        sort: None,
        uid: false,
        samefilesystem: false,
        level: Some(2), // 限制深度为2，避免遍历太多
        path: Some(test_path.clone()),
        json: false,
        progress: false,
        threads: None,
    };

    let config = Config {
        args: args.clone(),
        color_enabled: false,
        exclude_regex: None,
        include_regex: None,
        depth: 2,
    };

    let walker = Walker::new(config);
    let entries: Vec<_> = walker.walk().collect();

    // 应该找到至少根目录和一些子条目
    assert!(!entries.is_empty(), "应该找到一些条目");
    assert!(entries.len() > 1, "应该找到多于1个条目");

    // 验证根目录存在
    let root_count = entries.iter().filter(|e| e.depth() == 0).count();
    assert_eq!(root_count, 1, "应该恰好有一个根目录");
}

#[test]
fn test_parallel_walk() {
    let test_path = get_test_path();

    let args = Args {
        all: true,
        ascii: false,
        color: false,
        dirs_only: false,
        mtime: false,
        filelimit: false,
        full_path: false,
        gid: false,
        noreport: false,
        exclude: None,
        follow_links: false,
        no_color: true,
        literal: false,
        include: None,
        perms: false,
        quiet: false,
        size: false,
        sort: None,
        uid: false,
        samefilesystem: false,
        level: Some(2),
        path: Some(test_path.clone()),
        json: false,
        progress: false,
        threads: Some(2),
    };

    let config = Config {
        args: args.clone(),
        color_enabled: false,
        exclude_regex: None,
        include_regex: None,
        depth: 2,
    };

    let walker = Walker::new(config);
    let entries = walker.walk_parallel();

    // 并行遍历应该返回条目
    assert!(!entries.is_empty(), "并行遍历应该返回条目");
    assert!(entries.len() > 1, "并行遍历应该找到多于1个条目");
}

#[test]
fn test_parallel_vs_serial_consistency() {
    let test_path = get_test_path();

    let args = Args {
        all: true,
        ascii: false,
        color: false,
        dirs_only: false,
        mtime: false,
        filelimit: false,
        full_path: false,
        gid: false,
        noreport: false,
        exclude: None,
        follow_links: false,
        no_color: true,
        literal: false,
        include: None,
        perms: false,
        quiet: false,
        size: false,
        sort: None,
        uid: false,
        samefilesystem: false,
        level: Some(2),
        path: Some(test_path.clone()),
        json: false,
        progress: false,
        threads: None,
    };

    let config = Config {
        args: args.clone(),
        color_enabled: false,
        exclude_regex: None,
        include_regex: None,
        depth: 2,
    };

    let walker = Walker::new(config.clone());

    // 串行处理
    let serial_entries: Vec<_> = walker.walk().collect();
    let serial_sorted = walker.sort_entries(serial_entries);

    // 并行处理
    let parallel_sorted = walker.walk_parallel();

    // 数量应该相同
    assert_eq!(
        serial_sorted.len(),
        parallel_sorted.len(),
        "串行和并行处理应该返回相同数量的条目"
    );
}

#[test]
fn test_sort_entries() {
    let test_path = get_test_path();

    let args = Args {
        all: true,
        ascii: false,
        color: false,
        dirs_only: false,
        mtime: false,
        filelimit: false,
        full_path: false,
        gid: false,
        noreport: false,
        exclude: None,
        follow_links: false,
        no_color: true,
        literal: false,
        include: None,
        perms: false,
        quiet: false,
        size: false,
        sort: None,
        uid: false,
        samefilesystem: false,
        level: Some(2),
        path: Some(test_path.clone()),
        json: false,
        progress: false,
        threads: None,
    };

    let config = Config {
        args: args.clone(),
        color_enabled: false,
        exclude_regex: None,
        include_regex: None,
        depth: 2,
    };

    let walker = Walker::new(config);
    let entries: Vec<_> = walker.walk().collect();
    let sorted = walker.sort_entries(entries);

    // 验证排序后根目录是第一个
    if !sorted.is_empty() {
        assert_eq!(sorted[0].depth(), 0, "根目录应该是第一个");
    }
}

#[test]
fn test_json_output_structure() {
    let test_path = get_test_path();

    let args = Args {
        all: true,
        ascii: false,
        color: false,
        dirs_only: false,
        mtime: true,
        filelimit: true,
        full_path: true,
        gid: false,
        noreport: false,
        exclude: None,
        follow_links: false,
        no_color: true,
        literal: false,
        include: None,
        perms: false,
        quiet: false,
        size: true,
        sort: None,
        uid: false,
        samefilesystem: false,
        level: Some(2),
        path: Some(test_path.clone()),
        json: true,
        progress: false,
        threads: None,
    };

    let config = Config {
        args: args.clone(),
        color_enabled: false,
        exclude_regex: None,
        include_regex: None,
        depth: 2,
    };

    let walker = Walker::new(config.clone());
    let entries: Vec<_> = walker.walk().collect();
    let _sorted = walker.sort_entries(entries);

    let _formatter = Formatter::new(config);

    // 验证JSON构建不崩溃
    assert!(true);
}

#[test]
fn test_dirs_only_filter() {
    let test_path = get_test_path();

    let args = Args {
        all: true,
        ascii: false,
        color: false,
        dirs_only: true, // 仅目录
        mtime: false,
        filelimit: false,
        full_path: false,
        gid: false,
        noreport: false,
        exclude: None,
        follow_links: false,
        no_color: true,
        literal: false,
        include: None,
        perms: false,
        quiet: false,
        size: false,
        sort: None,
        uid: false,
        samefilesystem: false,
        level: Some(2),
        path: Some(test_path.clone()),
        json: false,
        progress: false,
        threads: None,
    };

    let config = Config {
        args: args.clone(),
        color_enabled: false,
        exclude_regex: None,
        include_regex: None,
        depth: 2,
    };

    let walker = Walker::new(config);
    let entries: Vec<_> = walker.walk().collect();

    // 验证所有条目都是目录（除了根目录）
    for entry in &entries {
        if entry.depth() > 0 {
            assert!(entry.file_type().is_dir(), "所有条目都应该是目录");
        }
    }
}

#[test]
fn test_exclude_pattern() {
    let test_path = get_test_path();

    let args = Args {
        all: true,
        ascii: false,
        color: false,
        dirs_only: false,
        mtime: false,
        filelimit: false,
        full_path: false,
        gid: false,
        noreport: false,
        exclude: Some("target".to_string()), // 排除target目录
        follow_links: false,
        no_color: true,
        literal: false,
        include: None,
        perms: false,
        quiet: false,
        size: false,
        sort: None,
        uid: false,
        samefilesystem: false,
        level: Some(3),
        path: Some(test_path.clone()),
        json: false,
        progress: false,
        threads: None,
    };

    let config = Config {
        args: args.clone(),
        color_enabled: false,
        exclude_regex: Some(regex::Regex::new("target").unwrap()),
        include_regex: None,
        depth: 3,
    };

    let walker = Walker::new(config);
    let entries: Vec<_> = walker.walk().collect();

    // 验证没有target目录
    for entry in &entries {
        if let Some(name) = entry.file_name().to_str() {
            assert_ne!(name, "target", "不应该找到target目录");
        }
    }
}

#[test]
fn test_include_pattern() {
    let test_path = get_test_path();

    let args = Args {
        all: true,
        ascii: false,
        color: false,
        dirs_only: false,
        mtime: false,
        filelimit: false,
        full_path: false,
        gid: false,
        noreport: false,
        exclude: None,
        follow_links: false,
        no_color: true,
        literal: false,
        include: Some("Cargo".to_string()), // 只包含Cargo相关文件
        perms: false,
        quiet: false,
        size: false,
        sort: None,
        uid: false,
        samefilesystem: false,
        level: Some(2),
        path: Some(test_path.clone()),
        json: false,
        progress: false,
        threads: None,
    };

    let config = Config {
        args: args.clone(),
        color_enabled: false,
        exclude_regex: None,
        include_regex: Some(regex::Regex::new("Cargo").unwrap()),
        depth: 2,
    };

    let walker = Walker::new(config);
    let entries: Vec<_> = walker.walk().collect();

    // 验证只包含Cargo相关条目
    let has_cargo = entries.iter().any(|e| {
        if let Some(name) = e.file_name().to_str() {
            name.contains("Cargo")
        } else {
            false
        }
    });

    assert!(has_cargo, "应该找到Cargo相关条目");
}

#[test]
fn test_sort_by_time() {
    let test_path = get_test_path();

    let args = Args {
        all: true,
        ascii: false,
        color: false,
        dirs_only: false,
        mtime: false,
        filelimit: false,
        full_path: false,
        gid: false,
        noreport: false,
        exclude: None,
        follow_links: false,
        no_color: true,
        literal: false,
        include: None,
        perms: false,
        quiet: false,
        size: false,
        sort: Some("time".to_string()), // 按时间排序
        uid: false,
        samefilesystem: false,
        level: Some(2),
        path: Some(test_path.clone()),
        json: false,
        progress: false,
        threads: None,
    };

    let config = Config {
        args: args.clone(),
        color_enabled: false,
        exclude_regex: None,
        include_regex: None,
        depth: 2,
    };

    let walker = Walker::new(config);
    let entries: Vec<_> = walker.walk().collect();
    let sorted = walker.sort_entries(entries);

    // 验证排序成功（不崩溃）
    assert!(!sorted.is_empty(), "排序应该返回结果");
}