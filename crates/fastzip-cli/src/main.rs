//! FastZip CLI - 跨平台快速解压缩工具

use std::path::PathBuf;

use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};

use fastzip_core::{
    compress_to_7z, compress_to_zip, extract_many, CompressOptions, ExtractOptions, FastZipError,
};

mod args;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("fastzip_core=info")),
        )
        .with_target(false)
        .try_init()
        .ok();

    if let Err(e) = run() {
        eprintln!("错误: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), FastZipError> {
    let cli = args::Cli::parse();

    match &cli.command {
        args::Command::Extract(a) | args::Command::X(a) => run_extract(a),
        args::Command::Compress(a) | args::Command::C(a) => run_compress(a),
    }
}


fn run_extract(extract_args: &args::ExtractArgs) -> Result<(), FastZipError> {
    let options = ExtractOptions {
        dest: extract_args.dest.clone(),
        smart: extract_args.smart(),
        overwrite: extract_args.overwrite,
        password: extract_args
            .password
            .clone()
            .or_else(|| std::env::var("FASTZIP_PASSWORD").ok()),
    };

    let archives: Vec<PathBuf> = extract_args
        .archive
        .iter()
        .filter_map(|p| {
            let pb = PathBuf::from(p);
            if pb.exists() {
                Some(pb)
            } else {
                eprintln!("警告: 文件不存在，已跳过: {}", p);
                None
            }
        })
        .collect();

    if archives.is_empty() {
        return Err(FastZipError::Other("没有有效的压缩文件".into()));
    }

    let pb = if !extract_args.quiet && archives.len() > 1 {
        Some(
            ProgressBar::new(archives.len() as u64).with_style(
                ProgressStyle::default_bar()
                    .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} {msg}")
                    .unwrap(),
            ),
        )
    } else {
        None
    };

    let results = extract_many(&archives, &options)?;

    for (i, result) in results.into_iter().enumerate() {
        if let Some(ref p) = pb {
            p.set_message(archives[i].file_name().unwrap_or_default().to_string_lossy().to_string());
            p.inc(1);
        }
        match result {
            Ok(dest) => {
                if !extract_args.quiet {
                    println!("已解压到: {}", dest.display());
                }
            }
            Err(e) => {
                eprintln!("解压失败 {}: {}", archives[i].display(), e);
            }
        }
    }

    if let Some(p) = pb {
        p.finish_with_message("完成");
    }

    Ok(())
}

fn run_compress(compress_args: &args::CompressArgs) -> Result<(), FastZipError> {
    let output = compress_args.output.as_ref().ok_or_else(|| {
        FastZipError::Other("压缩请指定输出文件：-o/--output <文件.zip 或 文件.7z>".into())
    })?;

    let sources: Vec<PathBuf> = compress_args
        .sources
        .iter()
        .map(PathBuf::from)
        .filter(|p| {
            if p.exists() {
                true
            } else {
                eprintln!("警告: 不存在，已跳过: {}", p.display());
                false
            }
        })
        .collect();

    if sources.is_empty() {
        return Err(FastZipError::Other("没有有效的源路径".into()));
    }

    let ext = output
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    let options = CompressOptions {
        recursive: compress_args.recursive,
        password: None,
        fast: !compress_args.no_fast,
    };

    if ext == "7z" {
        if sources.len() > 1 {
            return Err(FastZipError::Other(
                "7z 格式仅支持单一路径，请指定一个目录或文件".into(),
            ));
        }
        compress_to_7z(&sources[0], output)?;
    } else if ext == "zip" {
        compress_to_zip(&sources, output, &options)?;
    } else {
        return Err(FastZipError::Other(
            "仅支持 .zip 或 .7z 输出，请使用 -o 指定扩展名".into(),
        ));
    }

    if !compress_args.quiet {
        println!("已压缩到: {}", output.display());
    }
    Ok(())
}
