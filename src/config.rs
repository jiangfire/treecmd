use clap::Parser;
use std::path::PathBuf;
use atty;

/// 以树状结构递归显示目录内容的命令行工具
/// 兼容Linux tree命令参数
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// 显示所有文件和目录，包括隐藏文件
    #[arg(short = 'a', long = "all")]
    pub all: bool,
    
    /// 使用ASCII线条字符（└, ├, ─, │）
    #[arg(short = 'A', long = "ascii")]
    pub ascii: bool,
    
    /// 彩色输出文件名
    #[arg(short = 'C', long = "color")]
    pub color: bool,
    
    /// 仅显示目录，不显示文件
    #[arg(short = 'd', long = "dirs-only")]
    pub dirs_only: bool,
    
    /// 显示文件最后修改时间
    #[arg(short = 'D', long = "mtime")]
    pub mtime: bool,
    
    /// 在目录后添加"/"，可执行文件后添加"*"，符号链接后添加"@"
    #[arg(short = 'F', long = "filelimit")]
    pub filelimit: bool,
    
    /// 显示每个文件的完整路径前缀
    #[arg(short = 'f', long = "full-path")]
    pub full_path: bool,
    
    /// 显示文件所属组名称或GID
    #[arg(short = 'g', long = "gid")]
    pub gid: bool,
    
    /// 不显示缩进线，仅显示文件列表
    #[arg(short = 'i', long = "noreport")]
    pub noreport: bool,
    
    /// 排除匹配指定模式的文件和目录
    #[arg(short = 'I', long = "exclude")]
    pub exclude: Option<String>,
    
    /// 跟随符号链接，视为目录
    #[arg(short = 'l', long = "follow-links")]
    pub follow_links: bool,
    
    /// 不显示彩色输出
    #[arg(short = 'n', long = "no-color")]
    pub no_color: bool,
    
    /// 直接显示不可打印字符
    #[arg(short = 'N', long = "literal")]
    pub literal: bool,
    
    /// 仅显示匹配指定模式的文件和目录
    #[arg(short = 'P', long = "include")]
    pub include: Option<String>,
    
    /// 显示文件权限
    #[arg(short = 'p', long = "perms")]
    pub perms: bool,
    
    /// 用问号代替不可打印字符
    #[arg(short = 'q', long = "quiet")]
    pub quiet: bool,
    
    /// 显示每个文件的大小
    #[arg(short = 's', long = "size")]
    pub size: bool,
    
    /// 按修改时间排序，最新的在前
    #[arg(short = 't', long = "sort")]
    pub sort: Option<String>,
    
    /// 显示文件所有者名称或UID
    #[arg(short = 'u', long = "uid")]
    pub uid: bool,
    
    /// 仅遍历当前文件系统
    #[arg(short = 'x', long = "samefilesystem")]
    pub samefilesystem: bool,
    
    /// 限制显示的目录深度
    #[arg(short = 'L', long = "level")]
    pub level: Option<usize>,
    
    /// 起始目录（默认为当前目录）
    pub path: Option<PathBuf>,
    
    /// 增强功能：以JSON格式输出结果
    #[arg(long = "json")]
    pub json: bool,
    
    /// 增强功能：显示处理进度
    #[arg(long = "progress")]
    pub progress: bool,
    
    /// 增强功能：指定并行处理的线程数
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
