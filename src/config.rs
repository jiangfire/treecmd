use atty;
use clap::Parser;
use std::path::PathBuf;

/// 以树状结构递归显示目录内容的命令行工具
/// 兼容Linux tree命令参数
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Show all files and directories, including hidden ones
    #[arg(short = 'a', long = "all")]
    pub all: bool,

    /// Use ASCII line-drawing characters (└, ├, ─, │)
    #[arg(short = 'A', long = "ascii")]
    pub ascii: bool,

    /// Colorize output filenames
    #[arg(short = 'C', long = "color")]
    pub color: bool,

    /// List directories only, omit files
    #[arg(short = 'd', long = "dirs-only")]
    pub dirs_only: bool,

    /// Show last modification time of files
    #[arg(short = 'D', long = "mtime")]
    pub mtime: bool,

    /// Append '/' to directories, '*' to executables, '@' to symbolic links
    #[arg(short = 'F', long = "filelimit")]
    pub filelimit: bool,

    /// Print full path prefix for each file
    #[arg(short = 'f', long = "full-path")]
    pub full_path: bool,

    /// Show group name or GID for each file
    #[arg(short = 'g', long = "gid")]
    pub gid: bool,

    /// Don't print indentation lines, only file list
    #[arg(short = 'i', long = "noreport")]
    pub noreport: bool,

    /// Exclude files and directories matching the given pattern
    #[arg(short = 'I', long = "exclude")]
    pub exclude: Option<String>,

    /// Follow symbolic links as if they were directories
    #[arg(short = 'l', long = "follow-links")]
    pub follow_links: bool,

    /// Disable colorized output
    #[arg(short = 'n', long = "no-color")]
    pub no_color: bool,

    /// Print non-printable characters as-is
    #[arg(short = 'N', long = "literal")]
    pub literal: bool,

    /// Show only files and directories matching the given pattern
    #[arg(short = 'P', long = "include")]
    pub include: Option<String>,

    /// Show file permissions
    #[arg(short = 'p', long = "perms")]
    pub perms: bool,

    /// Replace non-printable characters with '?'
    #[arg(short = 'q', long = "quiet")]
    pub quiet: bool,

    /// Show size of each file
    #[arg(short = 's', long = "size")]
    pub size: bool,

    /// Sort files by modification time, newest first
    #[arg(short = 't', long = "sort")]
    pub sort: Option<String>,

    /// Show owner name or UID for each file
    #[arg(short = 'u', long = "uid")]
    pub uid: bool,

    /// Stay on the same filesystem
    #[arg(short = 'x', long = "samefilesystem")]
    pub samefilesystem: bool,

    /// Limit the depth of directories displayed
    #[arg(short = 'L', long = "level")]
    pub level: Option<usize>,

    /// Starting directory (defaults to current directory)
    pub path: Option<PathBuf>,

    /// Output results in JSON format
    #[arg(long = "json")]
    pub json: bool,

    /// Display processing progress
    #[arg(long = "progress")]
    pub progress: bool,

    /// Number of threads for parallel processing
    #[arg(long = "threads")]
    pub threads: Option<usize>,
}

/// 配置结构，存储处理后的命令行参数
#[derive(Debug, Clone)]
pub struct Config {
    pub args: Args,
    pub color_enabled: bool,
    pub exclude_regex: Option<regex::Regex>,
    pub include_regex: Option<regex::Regex>,
    pub depth: usize, // 处理后的目录深度限制
}

impl Config {
    /// 从命令行参数创建配置
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let args = Args::parse();

        // 设置默认深度限制
        const DEFAULT_DEPTH: usize = 3;
        let depth = args.level.unwrap_or(DEFAULT_DEPTH);

        // 处理彩色输出配置
        let color_enabled = if args.no_color {
            false
        } else if args.color {
            true
        } else {
            // 默认：如果stdout是终端，则启用彩色
            atty::is(atty::Stream::Stdout)
        };

        // 编译排除正则表达式
        let exclude_regex = match &args.exclude {
            Some(pattern) => Some(regex::Regex::new(pattern)?),
            None => None,
        };

        // 编译包含正则表达式
        let include_regex = match &args.include {
            Some(pattern) => Some(regex::Regex::new(pattern)?),
            None => None,
        };

        Ok(Self {
            args,
            color_enabled,
            exclude_regex,
            include_regex,
            depth,
        })
    }
}
