//! 错误类型定义

use std::path::PathBuf;
use thiserror::Error;

/// FastZip 错误类型
#[derive(Error, Debug)]
pub enum FastZipError {
    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),

    #[error("ZIP 格式错误: {0}")]
    Zip(#[from] zip::result::ZipError),

    #[error("7z 格式错误: {0}")]
    SevenZ(String),

    #[error("TAR 格式错误: {0}")]
    Tar(String),

    #[error("不支持的格式: {0}")]
    UnsupportedFormat(String),

    #[error("未找到文件: {0}")]
    FileNotFound(PathBuf),

    #[error("无法检测压缩格式")]
    FormatDetectionFailed,

    #[error("密码错误或需要密码")]
    PasswordRequired,

    #[error("{0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, FastZipError>;
