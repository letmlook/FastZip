//! Tauri 后端：对话框与 fastzip-core 集成

use std::path::PathBuf;

use fastzip_core::{
    compress_to_7z, compress_to_zip, extract_one, list_archive_top_level, ArchiveFormat,
    CompressOptions, ExtractOptions,
};
use tauri::command;

#[command]
fn pick_file() -> Option<String> {
    rfd::FileDialog::new()
        .pick_file()
        .map(|p| p.to_string_lossy().to_string())
}

#[command]
fn pick_folder() -> Option<String> {
    rfd::FileDialog::new()
        .pick_folder()
        .map(|p| p.to_string_lossy().to_string())
}

#[command]
fn pick_files() -> Option<Vec<String>> {
    rfd::FileDialog::new().pick_files().map(|v| {
        v.into_iter()
            .map(|p| p.to_string_lossy().to_string())
            .collect()
    })
}

#[command]
fn save_file() -> Option<String> {
    rfd::FileDialog::new()
        .add_filter("ZIP", &["zip"])
        .add_filter("7z", &["7z"])
        .save_file()
        .map(|p| p.to_string_lossy().to_string())
}

fn format_name(f: &ArchiveFormat) -> String {
    match f {
        ArchiveFormat::Zip => "ZIP",
        ArchiveFormat::SevenZ => "7z",
        ArchiveFormat::Rar => "RAR",
        ArchiveFormat::Tar => "TAR",
        ArchiveFormat::TarGz => "TAR.GZ",
        ArchiveFormat::TarXz => "TAR.XZ",
        ArchiveFormat::TarBz2 => "TAR.BZ2",
        ArchiveFormat::TarZst => "TAR.ZST",
        ArchiveFormat::Gz => "GZ",
        ArchiveFormat::Xz => "XZ",
        ArchiveFormat::Bz2 => "BZ2",
        ArchiveFormat::Zst => "ZST",
    }
    .to_string()
}

#[command]
fn list_archive(path: String) -> Result<(String, Vec<String>), String> {
    let p = PathBuf::from(&path);
    let (format, entries) = list_archive_top_level(&p).map_err(|e| e.to_string())?;
    Ok((format_name(&format), entries.entries))
}

#[command]
fn extract(
    archive: String,
    dest: String,
    smart: bool,
    password: Option<String>,
) -> Result<String, String> {
    let archive_path = PathBuf::from(&archive);
    let dest_path = PathBuf::from(&dest);
    let opts = ExtractOptions {
        dest: Some(dest_path.clone()),
        smart,
        overwrite: false,
        password,
    };
    let result_path = extract_one(&archive_path, &opts).map_err(|e| e.to_string())?;
    Ok(result_path.display().to_string())
}

#[command]
fn compress(
    sources: Vec<String>,
    dest: String,
    format_zip: bool,
    recursive: bool,
) -> Result<(), String> {
    let dest_path = PathBuf::from(&dest);
    let sources: Vec<PathBuf> = sources.into_iter().map(PathBuf::from).collect();
    if format_zip {
        compress_to_zip(
            &sources,
            &dest_path,
            &CompressOptions {
                recursive,
                password: None,
                fast: true,
            },
        )
        .map_err(|e| e.to_string())
    } else {
        if sources.len() > 1 {
            return Err("7z 仅支持单一路径".to_string());
        }
        compress_to_7z(&sources[0], &dest_path).map_err(|e| e.to_string())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            pick_file,
            pick_folder,
            pick_files,
            save_file,
            list_archive,
            extract,
            compress,
        ])
        .run(tauri::generate_context!())
        .expect("Tauri 应用启动失败");
}
