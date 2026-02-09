//! 解压引擎：统一调度各格式解压

use std::path::Path;

use rayon::prelude::*;
use tracing::{debug, info};

use crate::error::Result;
use crate::formats::{
    detect_format, extract_single_compressed, ArchiveFormat, SevenZExtractor, TarExtractor,
    ZipExtractor,
};
#[cfg(feature = "unrar")]
use crate::formats::RarExtractor;
use crate::smart_dest::resolve_smart_dest;

/// 解压选项
#[derive(Debug, Clone)]
pub struct ExtractOptions {
    /// 目标目录（未指定时使用压缩包所在目录）
    pub dest: Option<std::path::PathBuf>,
    /// 智能解压（默认 true）
    pub smart: bool,
    /// 覆盖已存在文件
    pub overwrite: bool,
    /// 密码（可选）
    pub password: Option<String>,
}

impl Default for ExtractOptions {
    fn default() -> Self {
        Self {
            dest: None,
            smart: true,
            overwrite: false,
            password: None,
        }
    }
}

/// 解压单个文件
pub fn extract_one(archive_path: &Path, options: &ExtractOptions) -> Result<std::path::PathBuf> {
    info!(path = %archive_path.display(), "开始解压");
    let format = detect_format(archive_path)?;
    debug!(format = ?format, "格式已检测");

    let base_dir = options
        .dest
        .clone()
        .unwrap_or_else(|| archive_path.parent().unwrap_or(Path::new(".")).to_path_buf());

    std::fs::create_dir_all(&base_dir)?;

    let dest_dir = if options.smart {
        resolve_smart_dest(archive_path, &base_dir, format)?
    } else {
        // flat 模式：解压到 base_dir，对于多文件归档使用 archive_stem 子目录
        if format.is_archive() {
            let stem = archive_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("extracted");
            let stem = if stem.ends_with(".tar") {
                stem.strip_suffix(".tar").unwrap_or(stem)
            } else {
                stem
            };
            base_dir.join(stem)
        } else {
            base_dir.clone()
        }
    };

    std::fs::create_dir_all(&dest_dir)?;
    info!(dest = %dest_dir.display(), "目标目录已确定");

    let password = options.password.as_deref();

    match format {
        ArchiveFormat::Zip => {
            ZipExtractor::extract(archive_path, &dest_dir, password)?;
        }
        ArchiveFormat::SevenZ => {
            SevenZExtractor::extract(archive_path, &dest_dir, password)?;
        }
        #[cfg(feature = "unrar")]
        ArchiveFormat::Rar => {
            RarExtractor::extract(archive_path, &dest_dir, password)?;
        }
        ArchiveFormat::Tar | ArchiveFormat::TarGz | ArchiveFormat::TarXz | ArchiveFormat::TarBz2 | ArchiveFormat::TarZst => {
            TarExtractor::extract(archive_path, &dest_dir, format)?;
        }
        ArchiveFormat::Gz | ArchiveFormat::Xz | ArchiveFormat::Bz2 | ArchiveFormat::Zst => {
            extract_single_compressed(archive_path, &dest_dir, format, options.overwrite)?;
        }
        #[cfg(not(feature = "unrar"))]
        ArchiveFormat::Rar => {
            return Err(crate::error::FastZipError::UnsupportedFormat(
                "RAR 格式需使用 full feature 编译".into(),
            ));
        }
    }

    info!(path = %archive_path.display(), dest = %dest_dir.display(), "解压完成");
    Ok(dest_dir)
}

/// 并行解压多个文件
pub fn extract_many<P: AsRef<Path> + Sync>(
    archive_paths: &[P],
    options: &ExtractOptions,
) -> Result<Vec<Result<std::path::PathBuf>>> {
    let results: Vec<_> = archive_paths
        .par_iter()
        .map(|p| extract_one(p.as_ref(), options))
        .collect();
    Ok(results)
}
