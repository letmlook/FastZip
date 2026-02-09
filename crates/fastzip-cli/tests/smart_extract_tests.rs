//! 智能解压逻辑测试：W6/W7 验收

use std::fs;
use std::io::Write;

use fastzip_core::{extract_one, ExtractOptions};
use tempfile::TempDir;
use zip::write::SimpleFileOptions;

fn create_multi_top_level_zip() -> (TempDir, std::path::PathBuf) {
    // 多顶层：a/file, b/file -> 应创建 archive_stem 子文件夹
    let dir = TempDir::new().unwrap();
    let zip_path = dir.path().join("multi.zip");

    let mut zip = zip::ZipWriter::new(std::fs::File::create(&zip_path).unwrap());
    zip.start_file("a/file1.txt", SimpleFileOptions::default()).unwrap();
    zip.write_all(b"a").unwrap();
    zip.start_file("b/file2.txt", SimpleFileOptions::default()).unwrap();
    zip.write_all(b"b").unwrap();
    zip.finish().unwrap();

    (dir, zip_path)
}

fn create_single_root_zip() -> (TempDir, std::path::PathBuf) {
    // 单根目录：foo/a, foo/b -> 应解压到 base_dir，创建 foo/
    let dir = TempDir::new().unwrap();
    let zip_path = dir.path().join("single_root.zip");

    let mut zip = zip::ZipWriter::new(std::fs::File::create(&zip_path).unwrap());
    zip.start_file("photos/img1.jpg", SimpleFileOptions::default()).unwrap();
    zip.write_all(b"img1").unwrap();
    zip.start_file("photos/img2.jpg", SimpleFileOptions::default()).unwrap();
    zip.write_all(b"img2").unwrap();
    zip.finish().unwrap();

    (dir, zip_path)
}

#[test]
fn test_smart_multi_top_level_creates_subfolder() {
    // W6: 多文件/多顶层 -> 创建 archive_stem 子文件夹
    let (dir, zip_path) = create_multi_top_level_zip();
    let dest_dir = dir.path().join("out");
    fs::create_dir_all(&dest_dir).unwrap();

    let options = ExtractOptions {
        dest: Some(dest_dir.clone()),
        smart: true,
        overwrite: false,
        password: None,
    };

    let result = extract_one(&zip_path, &options).unwrap();

    assert_eq!(result, dest_dir.join("multi"));
    assert!(result.join("a/file1.txt").exists());
    assert!(result.join("b/file2.txt").exists());
}

#[test]
fn test_smart_single_root_extracts_to_base() {
    // W6: 所有文件在同一根目录 -> 解压到 base_dir
    let (dir, zip_path) = create_single_root_zip();
    let dest_dir = dir.path().join("out");
    fs::create_dir_all(&dest_dir).unwrap();

    let options = ExtractOptions {
        dest: Some(dest_dir.clone()),
        smart: true,
        overwrite: false,
        password: None,
    };

    let result = extract_one(&zip_path, &options).unwrap();

    assert_eq!(result, dest_dir);
    assert!(dest_dir.join("photos/img1.jpg").exists());
    assert!(dest_dir.join("photos/img2.jpg").exists());
}

#[test]
fn test_smart_duplicate_folder_uses_2_and_3() {
    // W7: 重名文件夹 -> (2), (3) ...
    let (dir, zip_path) = create_multi_top_level_zip();
    let dest_dir = dir.path().join("out");
    fs::create_dir_all(&dest_dir).unwrap();

    let options = ExtractOptions {
        dest: Some(dest_dir.clone()),
        smart: true,
        overwrite: false,
        password: None,
    };

    let r1 = extract_one(&zip_path, &options).unwrap();
    assert_eq!(r1, dest_dir.join("multi"));

    let r2 = extract_one(&zip_path, &options).unwrap();
    assert_eq!(r2, dest_dir.join("multi (2)"));

    let r3 = extract_one(&zip_path, &options).unwrap();
    assert_eq!(r3, dest_dir.join("multi (3)"));

    assert!(r1.join("a/file1.txt").exists());
    assert!(r2.join("a/file1.txt").exists());
    assert!(r3.join("a/file1.txt").exists());
}
