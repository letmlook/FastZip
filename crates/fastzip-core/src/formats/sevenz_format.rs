//! 7z 格式解压

use std::path::Path;

use sevenz_rust::{decompress_file, decompress_file_with_password, Archive, Password};

use crate::error::{FastZipError, Result};
use crate::smart_dest::TopLevelEntries;

/// 7z 格式解压器
pub struct SevenZExtractor;

impl SevenZExtractor {
    /// 列出顶层条目
    pub fn list_top_level(path: &Path) -> Result<TopLevelEntries> {
        let archive = Archive::open(path)
            .map_err(|e| FastZipError::SevenZ(e.to_string()))?;

        let mut top_level: Vec<String> = archive
            .files
            .iter()
            .filter_map(|e| {
                let name = e.name();
                let name = name.trim_end_matches('/');
                if name.is_empty() {
                    return None;
                }
                let top = name.split(['/', '\\']).next()?.to_string();
                Some(top)
            })
            .collect();

        top_level.sort();
        top_level.dedup();

        let single_root_dir = if top_level.len() == 1 && archive.files.len() > 1 {
            Some(top_level[0].clone())
        } else {
            None
        };

        let single_file = archive.files.len() == 1
            && archive.files[0].name().trim_end_matches('/').split(['/', '\\']).count() <= 1;

        Ok(TopLevelEntries {
            entries: top_level,
            single_root_dir,
            single_file,
        })
    }

    /// 解压到指定目录
    pub fn extract(path: &Path, dest: &Path, password: Option<&str>) -> Result<()> {
        if let Some(pwd) = password {
            decompress_file_with_password(path, dest, Password::from(pwd))
                .map_err(|e: sevenz_rust::Error| FastZipError::SevenZ(e.to_string()))?;
        } else {
            decompress_file(path, dest).map_err(|e: sevenz_rust::Error| FastZipError::SevenZ(e.to_string()))?;
        }
        Ok(())
    }
}
