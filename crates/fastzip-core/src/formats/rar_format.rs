//! RAR 格式解压（需启用 feature "full"）

#![cfg(feature = "unrar")]

use std::path::Path;

use unrar::Archive;

use crate::error::{FastZipError, Result};
use crate::smart_dest::TopLevelEntries;

/// RAR 格式解压器
pub struct RarExtractor;

impl RarExtractor {
    /// 列出顶层条目
    pub fn list_top_level(path: &Path) -> Result<TopLevelEntries> {
        let archive = Archive::new(path)
            .as_first_part()
            .open_for_listing()
            .map_err(|e| FastZipError::Other(format!("RAR 列出失败: {}", e)))?;

        let mut top_level: Vec<String> = Vec::new();
        let mut count = 0;
        let mut single_file = false;
        let mut single_root_dir: Option<String> = None;

        for entry in archive {
            let e = entry.map_err(|err| FastZipError::Other(format!("RAR: {}", err)))?;
            count += 1;
            let name = e.filename.to_string_lossy();
            let name = name.trim_end_matches('/').trim_end_matches('\\');
            if name.is_empty() {
                continue;
            }
            let top = name
                .split(['/', '\\'])
                .next()
                .unwrap_or("")
                .to_string();
            if !top.is_empty() && !top_level.contains(&top) {
                top_level.push(top);
            }
        }

        top_level.sort();
        top_level.dedup();

        if count == 1 && top_level.len() == 1 {
            let first = top_level[0].clone();
            if !first.contains('/') && !first.contains('\\') {
                single_file = true;
            }
        }
        if count > 1 && top_level.len() == 1 {
            single_root_dir = Some(top_level[0].clone());
        }

        Ok(TopLevelEntries {
            entries: top_level,
            single_root_dir,
            single_file,
        })
    }

    /// 解压到指定目录
    pub fn extract(path: &Path, dest: &Path, password: Option<&str>) -> Result<()> {
        let archive = if let Some(pwd) = password {
            Archive::with_password(path, pwd.as_bytes())
        } else {
            Archive::new(path)
        };

        let mut open = archive
            .as_first_part()
            .open_for_processing()
            .map_err(|e| FastZipError::Other(format!("RAR 打开失败: {}", e)))?;

        loop {
            let next = open.read_header();
            let next = match next {
                Ok(Some(n)) => n,
                Ok(None) => break,
                Err(e) => return Err(FastZipError::Other(format!("RAR 读取: {}", e))),
            };
            open = next
                .extract_with_base(dest)
                .map_err(|e| FastZipError::Other(format!("RAR 解压: {}", e)))?;
        }

        Ok(())
    }
}
