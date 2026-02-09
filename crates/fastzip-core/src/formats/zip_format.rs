//! ZIP 格式解压（含加密 ZIP 密码解压）

use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use zip::ZipArchive;

use crate::error::{FastZipError, Result};
use crate::smart_dest::TopLevelEntries;

/// ZIP 格式解压器
pub struct ZipExtractor;

impl ZipExtractor {
    /// 列出顶层条目（用于智能解压决策）
    /// 若归档加密且未提供密码，可能无法正确列出，调用方需处理
    pub fn list_top_level(path: &Path) -> Result<TopLevelEntries> {
        let file = File::open(path)?;
        let mut archive = ZipArchive::new(file)?;
        let len = archive.len();
        let mut entries = Vec::new();

        for i in 0..len {
            let entry = archive.by_index(i).or_else(|e| {
                if let zip::result::ZipError::InvalidPassword = e {
                    Err(FastZipError::PasswordRequired)
                } else {
                    Err(e.into())
                }
            })?;
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
            let e = a.by_index(0).or_else(|e| {
                if let zip::result::ZipError::InvalidPassword = e {
                    Err(FastZipError::PasswordRequired)
                } else {
                    Err(e.into())
                }
            })?;
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

    /// 解压到指定目录；若提供 password 则支持加密 ZIP（ZipCrypto/AES）
    pub fn extract(path: &Path, dest: &Path, password: Option<&str>) -> Result<()> {
        let file = File::open(path)?;
        let mut archive = ZipArchive::new(file)?;

        if let Some(pwd) = password {
            Self::extract_with_password(&mut archive, dest, pwd)
        } else {
            archive.extract(dest)?;
            Ok(())
        }
    }

    /// 使用密码逐条解压（兼容加密与非加密条目）
    fn extract_with_password(
        archive: &mut ZipArchive<File>,
        dest: &Path,
        password: &str,
    ) -> Result<()> {
        let pwd_bytes = password.as_bytes();
        for i in 0..archive.len() {
            let mut file = archive
                .by_index_decrypt(i, pwd_bytes)
                .or_else(|e| {
                    if let zip::result::ZipError::InvalidPassword = e {
                        Err(FastZipError::PasswordRequired)
                    } else {
                        Err(e.into())
                    }
                })?;

            let out_path = if let Some(enclosed) = file.enclosed_name() {
                dest.join(enclosed)
            } else {
                dest.join(file.mangled_name())
            };

            if file.is_dir() {
                fs::create_dir_all(&out_path)?;
            } else {
                if let Some(p) = out_path.parent() {
                    fs::create_dir_all(p)?;
                }
                let mut out_file = File::create(&out_path)?;
                std::io::copy(&mut file, &mut out_file)?;
                out_file.flush()?;
            }
        }
        Ok(())
    }
}
