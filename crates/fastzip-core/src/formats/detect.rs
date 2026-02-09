//! 格式检测：按扩展名与魔数

use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::error::{FastZipError, Result};

/// 支持的压缩/归档格式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchiveFormat {
    Zip,
    SevenZ,
    TarGz,
    TarXz,
    TarBz2,
    TarZst,
    Tar,  // 无压缩的 tar
    Gz,   // 单文件 gzip
    Xz,   // 单文件 xz
    Bz2,  // 单文件 bzip2
    Zst,  // 单文件 zstd
}

impl ArchiveFormat {
    /// 根据扩展名检测格式
    pub fn from_extension(path: &Path) -> Option<Self> {
        let ext = path.extension().and_then(|e| e.to_str())?.to_lowercase();
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_lowercase();

        // 处理双扩展名：archive.tar.gz
        if stem.ends_with(".tar") {
            return match ext.as_str() {
                "gz" | "tgz" => Some(Self::TarGz),
                "xz" | "txz" => Some(Self::TarXz),
                "bz2" | "tbz2" | "tb2" => Some(Self::TarBz2),
                "zst" | "tzst" => Some(Self::TarZst),
                _ => None,
            };
        }
        if stem.ends_with(".tgz") {
            return Some(Self::TarGz);
        }

        match ext.as_str() {
            "zip" => Some(Self::Zip),
            "7z" => Some(Self::SevenZ),
            "tar" => Some(Self::Tar),
            "tgz" => Some(Self::TarGz),
            "txz" => Some(Self::TarXz),
            "tbz2" | "tb2" => Some(Self::TarBz2),
            "tzst" => Some(Self::TarZst),
            "gz" | "gzip" => Some(Self::Gz),
            "xz" | "lzma" => Some(Self::Xz),
            "bz2" | "bzip2" => Some(Self::Bz2),
            "zst" | "zstd" => Some(Self::Zst),
            _ => None,
        }
    }

    /// 根据文件魔数检测格式
    pub fn from_magic(buf: &[u8]) -> Option<Self> {
        if buf.len() < 6 {
            return None;
        }
        // ZIP: PK\x03\x04 or PK\x05\x06 (empty) or PK\x07\x08 (spanned)
        if buf.starts_with(b"PK\x03\x04")
            || buf.starts_with(b"PK\x05\x06")
            || buf.starts_with(b"PK\x07\x08")
        {
            return Some(Self::Zip);
        }
        // 7z: 7z BC AF 27 1C
        if buf.starts_with(b"7z\xbc\xaf\x27\x1c") {
            return Some(Self::SevenZ);
        }
        // gzip: 1f 8b
        if buf.starts_with(b"\x1f\x8b") {
            return Some(Self::Gz);
        }
        // xz: FD 37 7A 58 5A 00
        if buf.starts_with(b"\xfd7zXZ\x00") {
            return Some(Self::Xz);
        }
        // bzip2: BZ
        if buf.starts_with(b"BZ") {
            return Some(Self::Bz2);
        }
        // zstd: 28 B5 2F FD
        if buf.starts_with(b"\x28\xb5\x2f\xfd") {
            return Some(Self::Zst);
        }
        // tar 无魔术，但前 257 字节后可能有多字节 ustar
        if buf.len() >= 262 {
            if &buf[257..262] == b"ustar" {
                return Some(Self::Tar);
            }
        }
        None
    }

    /// 是否为归档格式（多文件容器）
    pub fn is_archive(&self) -> bool {
        matches!(
            self,
            Self::Zip
                | Self::SevenZ
                | Self::Tar
                | Self::TarGz
                | Self::TarXz
                | Self::TarBz2
                | Self::TarZst
        )
    }

    /// 是否为单文件压缩格式
    pub fn is_single_compressed(&self) -> bool {
        matches!(self, Self::Gz | Self::Xz | Self::Bz2 | Self::Zst)
    }
}

/// 检测文件格式：优先扩展名，扩展名无法识别时用魔数
pub fn detect_format(path: &Path) -> Result<ArchiveFormat> {
    if let Some(fmt) = ArchiveFormat::from_extension(path) {
        return Ok(fmt);
    }
    // 魔数检测
    let mut f = File::open(path).map_err(|_| FastZipError::FileNotFound(path.to_path_buf()))?;
    let mut buf = [0u8; 32];
    let n = f.read(&mut buf)?;
    if let Some(fmt) = ArchiveFormat::from_magic(&buf[..n]) {
        return Ok(fmt);
    }
    Err(FastZipError::FormatDetectionFailed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_from_extension() {
        assert_eq!(
            ArchiveFormat::from_extension(Path::new("a.zip")),
            Some(ArchiveFormat::Zip)
        );
        assert_eq!(
            ArchiveFormat::from_extension(Path::new("a.7z")),
            Some(ArchiveFormat::SevenZ)
        );
        assert_eq!(
            ArchiveFormat::from_extension(Path::new("a.tar.gz")),
            Some(ArchiveFormat::TarGz)
        );
        assert_eq!(
            ArchiveFormat::from_extension(Path::new("a.tgz")),
            Some(ArchiveFormat::TarGz)
        );
        assert_eq!(
            ArchiveFormat::from_extension(Path::new("a.gz")),
            Some(ArchiveFormat::Gz)
        );
        assert_eq!(
            ArchiveFormat::from_extension(Path::new("a.xz")),
            Some(ArchiveFormat::Xz)
        );
        assert_eq!(
            ArchiveFormat::from_extension(Path::new("a.bz2")),
            Some(ArchiveFormat::Bz2)
        );
        assert_eq!(
            ArchiveFormat::from_extension(Path::new("a.zst")),
            Some(ArchiveFormat::Zst)
        );
    }
}
