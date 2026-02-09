//! 智能解压目标路径决策（参考 Bandizip Extract Here Smart）

use std::path::{Path, PathBuf};

use crate::error::Result;
use crate::formats::{detect_format, ArchiveFormat, SevenZExtractor, TarExtractor, ZipExtractor};

#[cfg(feature = "unrar")]
use crate::formats::RarExtractor;

/// 列出归档顶层条目（用于预览等），返回格式与条目信息
pub fn list_archive_top_level(path: &Path) -> Result<(ArchiveFormat, TopLevelEntries)> {
    let format = detect_format(path)?;
    let entries = list_top_level_entries(path, format)?;
    Ok((format, entries))
}

/// 顶层条目信息（用于智能解压决策）
#[derive(Debug, Clone)]
pub struct TopLevelEntries {
    /// 顶层条目名称列表（文件或目录）
    pub entries: Vec<String>,
    /// 若所有条目都在同一根目录下，则为该目录名
    pub single_root_dir: Option<String>,
    /// 是否为单文件归档（归档内只有一个文件且位于根）
    pub single_file: bool,
}

/// 计算智能解压的目标目录
///
/// 规则（与 Bandizip 一致）：
/// 1. 单文件 → base_dir
/// 2. 所有文件在同一根文件夹内 → base_dir（解压时会创建该根文件夹）
/// 3. 其他情况 → base_dir / (archive_stem) 或 base_dir / (archive_stem) (2) ...
pub fn resolve_smart_dest(archive_path: &Path, base_dir: &Path, format: ArchiveFormat) -> Result<PathBuf> {
    let entries = list_top_level_entries(archive_path, format)?;

    // 1. 单文件 → 当前目录
    if entries.single_file {
        return Ok(base_dir.to_path_buf());
    }

    // 2. 所有条目在同一根目录下 → 当前目录
    if entries.single_root_dir.is_some() {
        return Ok(base_dir.to_path_buf());
    }

    // 3. 其他情况：创建 archive_stem 文件夹，重名则 (2), (3)...
    let stem = archive_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("extracted")
        .to_string();

    // 处理 .tar.gz 等双扩展名：archive.tar.gz -> archive
    let stem = if stem.ends_with(".tar") {
        stem.strip_suffix(".tar").unwrap_or(&stem).to_string()
    } else {
        stem
    };

    let candidate = base_dir.join(&stem);
    if !candidate.exists() {
        return Ok(candidate);
    }

    let mut k = 2u32;
    loop {
        let next = base_dir.join(format!("{} ({})", stem, k));
        if !next.exists() {
            return Ok(next);
        }
        k += 1;
    }
}

/// 根据格式列出顶层条目
fn list_top_level_entries(path: &Path, format: ArchiveFormat) -> Result<TopLevelEntries> {
    if format.is_single_compressed() {
        // 单文件压缩格式只有一个"条目"
        return Ok(TopLevelEntries {
            entries: vec![],
            single_root_dir: None,
            single_file: true,
        });
    }

    match format {
        ArchiveFormat::Zip => ZipExtractor::list_top_level(path),
        ArchiveFormat::SevenZ => SevenZExtractor::list_top_level(path),
        #[cfg(feature = "unrar")]
        ArchiveFormat::Rar => RarExtractor::list_top_level(path),
        ArchiveFormat::Tar | ArchiveFormat::TarGz | ArchiveFormat::TarXz | ArchiveFormat::TarBz2 | ArchiveFormat::TarZst => {
            TarExtractor::list_top_level(path, format)
        }
        _ => Err(crate::error::FastZipError::UnsupportedFormat(format!("{:?}", format))),
    }
}
