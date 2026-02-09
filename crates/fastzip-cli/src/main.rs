//! FastZip CLI - 跨平台快速解压缩工具

use std::path::PathBuf;

use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};

use fastzip_core::{extract_many, ExtractOptions, FastZipError};

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

    let extract_args = match &cli.command {
        args::Command::Extract(a) | args::Command::X(a) => a,
    };

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
