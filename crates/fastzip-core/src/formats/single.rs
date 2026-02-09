//! 单文件压缩格式解压（.gz, .xz, .bz2, .zst）

use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::Path;

use bzip2::read::BzDecoder;
use flate2::read::GzDecoder;
use xz2::read::XzDecoder;
use zstd::Decoder;

use crate::error::{FastZipError, Result};
use crate::formats::ArchiveFormat;

/// 解压单文件压缩格式到指定目录
/// 输出文件名 = 输入文件名去掉压缩扩展名
pub fn extract_single_compressed(
    path: &Path,
    dest_dir: &Path,
    format: ArchiveFormat,
    overwrite: bool,
) -> Result<std::path::PathBuf> {
    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("output");
    // 对于 .tar.gz，file_stem 可能是 "archive.tar"，需要再去掉 .tar
    let out_name = if stem.ends_with(".tar") {
        stem.strip_suffix(".tar").unwrap_or(stem)
    } else {
        stem
    };

    if out_name.is_empty() {
        return Err(FastZipError::Other("无法确定输出文件名".into()));
    }

    let out_path = dest_dir.join(out_name);

    if !overwrite && out_path.exists() {
        return Ok(out_path);
    }

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut decoder: Box<dyn Read> = match format {
        ArchiveFormat::Gz => Box::new(GzDecoder::new(reader)),
        ArchiveFormat::Xz => Box::new(XzDecoder::new(reader)),
        ArchiveFormat::Bz2 => Box::new(BzDecoder::new(reader)),
        ArchiveFormat::Zst => Box::new(Decoder::new(reader)?),
        _ => return Err(FastZipError::UnsupportedFormat(format!("{:?}", format))),
    };

    let mut out_file = File::create(&out_path)?;
    std::io::copy(&mut decoder, &mut out_file)?;
    out_file.flush()?;

    Ok(out_path)
}
