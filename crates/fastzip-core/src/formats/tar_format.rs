//! TAR 格式解压（含 tar.gz, tar.xz, tar.bz2, tar.zst）

use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use bzip2::read::BzDecoder;
use flate2::read::GzDecoder;
use tar::Archive;
use xz2::read::XzDecoder;
use zstd::Decoder;

use crate::error::{FastZipError, Result};
use crate::formats::ArchiveFormat;
use crate::smart_dest::TopLevelEntries;

/// TAR 格式解压器
pub struct TarExtractor;

impl TarExtractor {
    /// 列出顶层条目
    pub fn list_top_level(path: &Path, format: ArchiveFormat) -> Result<TopLevelEntries> {
        let reader = Self::open_decoder(path, format)?;
        let mut archive = Archive::new(reader);

        let mut top_level: Vec<String> = Vec::new();
        let mut count = 0;
        let mut has_single_file_at_root = false;

        for entry in archive.entries().map_err(|e| FastZipError::Tar(e.to_string()))? {
            let e = entry.map_err(|e| FastZipError::Tar(e.to_string()))?;
            count += 1;
            let path = e.path().map_err(|e| FastZipError::Tar(e.to_string()))?;
            let path_str = path.to_string_lossy();
            let parts: Vec<&str> = path_str.split(['/', '\\']).collect();
            if parts.is_empty() || parts[0].is_empty() {
                continue;
            }
            let top = parts[0].to_string();
            if !top_level.contains(&top) {
                top_level.push(top);
            }
            if count == 1 && parts.len() == 1 && !path_str.ends_with('/') {
                has_single_file_at_root = true;
            } else if count > 1 {
                has_single_file_at_root = false;
            }
        }

        top_level.sort();
        top_level.dedup();

        let single_file = count == 1 && has_single_file_at_root;
        let single_root_dir = if top_level.len() == 1 && count > 1 {
            Some(top_level[0].clone())
        } else {
            None
        };

        Ok(TopLevelEntries {
            entries: top_level,
            single_root_dir,
            single_file,
        })
    }

    fn open_decoder(path: &Path, format: ArchiveFormat) -> Result<Box<dyn Read + Send>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let decoder: Box<dyn Read + Send> = match format {
            ArchiveFormat::Tar => Box::new(reader),
            ArchiveFormat::TarGz => Box::new(GzDecoder::new(reader)),
            ArchiveFormat::TarXz => Box::new(XzDecoder::new(reader)),
            ArchiveFormat::TarBz2 => Box::new(BzDecoder::new(reader)),
            ArchiveFormat::TarZst => Box::new(Decoder::new(reader)?),
            _ => return Err(FastZipError::UnsupportedFormat(format!("{:?}", format))),
        };
        Ok(decoder)
    }

    /// 解压到指定目录
    pub fn extract(path: &Path, dest: &Path, format: ArchiveFormat) -> Result<()> {
        let reader = Self::open_decoder(path, format)?;
        let mut archive = Archive::new(reader);
        archive.unpack(dest).map_err(|e| FastZipError::Tar(e.to_string()))?;
        Ok(())
    }
}
