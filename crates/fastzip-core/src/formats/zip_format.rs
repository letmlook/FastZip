//! ZIP 格式解压

use std::fs::File;
use std::path::Path;

use zip::ZipArchive;

use crate::error::Result;
use crate::smart_dest::TopLevelEntries;

/// ZIP 格式解压器
pub struct ZipExtractor;

impl ZipExtractor {
    /// 列出顶层条目（用于智能解压决策）
    pub fn list_top_level(path: &Path) -> Result<TopLevelEntries> {
        let file = File::open(path)?;
        let mut archive = ZipArchive::new(file)?;
        let len = archive.len();
        let mut entries = Vec::new();

        for i in 0..len {
            let entry = archive.by_index(i)?;
            let name = entry.name().to_string();
            let trimmed = name.trim_end_matches('/');
            if trimmed.is_empty() {
                continue;
            }
            let top = trimmed.split('/').next().unwrap_or("").to_string();
            if !top.is_empty() && !entries.contains(&top) {
                entries.push(top);
            }
        }

        entries.sort();
        entries.dedup();

        let single_file = if len == 1 {
            let mut a = ZipArchive::new(File::open(path)?)?;
            let e = a.by_index(0)?;
            !e.name().contains('/') && !e.name().ends_with('/')
        } else {
            false
        };

        let single_root_dir = if entries.len() == 1 && len > 1 {
            Some(entries[0].clone())
        } else {
            None
        };

        Ok(TopLevelEntries {
            entries,
            single_root_dir,
            single_file,
        })
    }

    /// 解压到指定目录
    pub fn extract(path: &Path, dest: &Path, _password: Option<&str>) -> Result<()> {
        let file = File::open(path)?;
        let mut archive = ZipArchive::new(file)?;
        archive.extract(dest)?;
        Ok(())
    }
}
