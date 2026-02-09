//! 路径与文件名规范化（Unicode/UTF-8 友好）

use std::path::{Component, Path, PathBuf};

/// 将归档内条目名规范化为安全相对路径（无 `..`、无绝对路径、统一分隔符）
/// 用于解压时避免路径穿越，并尽量保持 UTF-8 文件名
#[inline]
pub fn normalize_entry_path(entry_name: &str) -> PathBuf {
    let mut buf = PathBuf::new();
    for comp in Path::new(entry_name).components() {
        match comp {
            Component::Prefix(_) | Component::RootDir => {}
            Component::CurDir => {}
            Component::ParentDir => {
                buf.pop();
            }
            Component::Normal(s) => {
                if let Some(s) = s.to_str() {
                    buf.push(s);
                }
            }
        }
    }
    buf
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_no_escape() {
        let p = normalize_entry_path("foo/../etc/passwd");
        assert_eq!(p.file_name().unwrap().to_string_lossy(), "passwd");
        assert_eq!(p.components().count(), 2);
    }

    #[test]
    fn test_normalize_absolute() {
        let p = normalize_entry_path("/foo/bar");
        assert_eq!(p.file_name().unwrap().to_string_lossy(), "bar");
        assert_eq!(p.components().count(), 2);
    }
}
