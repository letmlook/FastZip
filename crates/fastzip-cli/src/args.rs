//! CLI 参数解析

use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "fastzip")]
#[command(author, version, about = "跨平台快速解压缩工具")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// 解压压缩文件
    Extract(ExtractArgs),
    /// 解压压缩文件（extract 的简写）
    X(ExtractArgs),
}

#[derive(Parser, Debug)]
pub struct ExtractArgs {
    /// 压缩文件路径（可指定多个）
    #[arg(required = true)]
    pub archive: Vec<String>,

    /// 解压目标目录
    #[arg(short, long, value_name = "DIR")]
    pub dest: Option<PathBuf>,

    /// 智能解压（默认）：根据内容自动选择目标路径
    #[arg(short, long, default_value_t = true)]
    pub smart: bool,

    /// 解压到此处（flat）：不使用智能解压，直接解压到目标目录
    #[arg(short, long, conflicts_with = "smart")]
    pub flat: bool,

    /// 覆盖已存在的文件
    #[arg(short, long)]
    pub overwrite: bool,

    /// 密码（也可通过 FASTZIP_PASSWORD 环境变量设置）
    #[arg(short, long)]
    pub password: Option<String>,

    /// 静默模式，不输出进度和路径
    #[arg(short, long)]
    pub quiet: bool,
}

impl ExtractArgs {
    pub fn smart(&self) -> bool {
        if self.flat {
            false
        } else {
            self.smart
        }
    }
}
