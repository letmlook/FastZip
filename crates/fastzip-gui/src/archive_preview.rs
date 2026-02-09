//! 归档预览：列出顶层条目

use std::path::Path;

use fastzip_core::{list_archive_top_level, ArchiveFormat};

/// 返回 (格式名称, 顶层条目名列表)
pub fn list_top_level(path: &Path) -> Result<(String, Vec<String>), fastzip_core::FastZipError> {
    let (format, entries) = list_archive_top_level(path)?;
    let name = format_name(&format);
    Ok((name, entries.entries))
}

fn format_name(f: &ArchiveFormat) -> String {
    match f {
        ArchiveFormat::Zip => "ZIP",
        ArchiveFormat::SevenZ => "7z",
        ArchiveFormat::Rar => "RAR",
        ArchiveFormat::Tar => "TAR",
        ArchiveFormat::TarGz => "TAR.GZ",
        ArchiveFormat::TarXz => "TAR.XZ",
        ArchiveFormat::TarBz2 => "TAR.BZ2",
        ArchiveFormat::TarZst => "TAR.ZST",
        ArchiveFormat::Gz => "GZ",
        ArchiveFormat::Xz => "XZ",
        ArchiveFormat::Bz2 => "BZ2",
        ArchiveFormat::Zst => "ZST",
    }
    .to_string()
}
