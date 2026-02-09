//! 压缩：创建 ZIP / 7z 归档

use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;

use walkdir::WalkDir;
use zip::write::SimpleFileOptions;
use zip::CompressionMethod;
use zip::ZipWriter;

use crate::error::{FastZipError, Result};

/// 压缩选项
#[derive(Debug, Clone)]
pub struct CompressOptions {
    /// 是否递归包含子目录
    pub recursive: bool,
    /// 密码（可选，仅 ZIP 支持）
    pub password: Option<String>,
    /// 快速模式：ZIP 仅存储不压缩（最快，体积较大）；关闭则使用 Deflate 压缩
    pub fast: bool,
}

impl Default for CompressOptions {
    fn default() -> Self {
        Self {
            recursive: true,
            password: None,
            fast: true,
        }
    }
}

/// 将若干路径打包为 ZIP
pub fn compress_to_zip<P: AsRef<Path>>(
    sources: &[P],
    dest: &Path,
    options: &CompressOptions,
) -> Result<()> {
    let file = File::create(dest)?;
    let mut zip = ZipWriter::new(BufWriter::with_capacity(1024 * 1024, file));
    let opts = zip_file_options(options.fast);

    for source in sources {
        let src = source.as_ref();
        if !src.exists() {
            return Err(FastZipError::FileNotFound(src.to_path_buf()));
        }
        if src.is_file() {
            let name = src.file_name().unwrap_or_default().to_string_lossy();
            add_file_to_zip(&mut zip, src, name.as_ref(), opts)?;
        } else if options.recursive {
            for entry in WalkDir::new(src).into_iter().filter_map(|e| e.ok()) {
                let path = entry.path();
                let rel = path.strip_prefix(src).unwrap_or(path);
                if path.is_dir() {
                    let name = rel.to_string_lossy().replace('\\', "/") + "/";
                    zip.add_directory(name, opts)?;
                } else {
                    let name = rel.to_string_lossy().replace('\\', "/");
                    add_file_to_zip(&mut zip, path, name.as_str(), opts)?;
                }
            }
        } else {
            let dir_name = src.file_name().unwrap_or_default().to_string_lossy().to_string() + "/";
            zip.add_directory(dir_name, opts)?;
        }
    }

    zip.finish()?;
    Ok(())
}

fn zip_file_options(fast: bool) -> SimpleFileOptions {
    if fast {
        // 仅存储不压缩，速度接近纯拷贝，体积等于原文件之和
        SimpleFileOptions::default().compression_method(CompressionMethod::Stored)
    } else {
        SimpleFileOptions::default()
    }
}

fn add_file_to_zip<W: Write + std::io::Seek>(
    zip: &mut ZipWriter<W>,
    path: &Path,
    name: &str,
    opts: SimpleFileOptions,
) -> Result<()> {
    let f = File::open(path)?;
    let mut reader = BufReader::with_capacity(1024 * 1024, f);
    zip.start_file(name, opts)?;
    std::io::copy(&mut reader, zip)?;
    Ok(())
}

/// 将单个路径压缩为 7z（目录或文件）
pub fn compress_to_7z<P: AsRef<Path>>(source: P, dest: &Path) -> Result<()> {
    let src = source.as_ref();
    if !src.exists() {
        return Err(FastZipError::FileNotFound(src.to_path_buf()));
    }
    sevenz_rust::compress_to_path(src, dest)
        .map_err(|e| FastZipError::SevenZ(e.to_string()))?;
    Ok(())
}
