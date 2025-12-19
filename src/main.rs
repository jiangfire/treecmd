mod config;
mod walker;
mod formatter;

use crate::config::Config;
use crate::formatter::Formatter;
use crate::walker::Walker;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建配置实例
    let config = Config::new()?;
    
    // 创建目录遍历器
    let walker = Walker::new(config.clone());
    
    // 执行目录遍历
    let entries = walker.walk();
    let sorted_entries = walker.sort_entries(entries.collect());
    
    // 创建输出格式化器
    let mut formatter = Formatter::new(config.clone());
    
    // 根据配置选择输出格式
    if config.args.json {
        formatter.format_json(sorted_entries.into_iter());
    } else {
        formatter.format_tree(sorted_entries.into_iter());
    }
    
    Ok(())
}