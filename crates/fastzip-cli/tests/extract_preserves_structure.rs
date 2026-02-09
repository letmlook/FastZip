//! 验证解压时多层目录结构是否保持

use std::fs;
use std::io::Write;

use fastzip_core::{extract_one, ExtractOptions};
use tempfile::TempDir;
use zip::write::SimpleFileOptions;

fn create_nested_zip() -> (TempDir, std::path::PathBuf) {
    let dir = TempDir::new().unwrap();
    let zip_path = dir.path().join("nested.zip");

    let mut zip = zip::ZipWriter::new(std::fs::File::create(&zip_path).unwrap());

    zip.start_file("root.txt", SimpleFileOptions::default()).unwrap();
    zip.write_all(b"root file").unwrap();

    zip.start_file("a/b/file1.txt", SimpleFileOptions::default()).unwrap();
    zip.write_all(b"level 2").unwrap();

    zip.start_file("a/b/c/file2.txt", SimpleFileOptions::default()).unwrap();
    zip.write_all(b"level 3").unwrap();

    zip.start_file("x/y/z/deep.txt", SimpleFileOptions::default()).unwrap();
    zip.write_all(b"deep nested").unwrap();

    zip.finish().unwrap();

    (dir, zip_path)
}

#[test]
fn test_nested_directories_preserved() {
    let (dir, zip_path) = create_nested_zip();
    let dest_dir = dir.path().join("out");

    let options = ExtractOptions {
        dest: Some(dest_dir.clone()),
        smart: true,
        overwrite: false,
        password: None,
    };

    let result = extract_one(&zip_path, &options).unwrap();

    // 验证根目录文件
    assert!(result.join("root.txt").exists(), "root.txt 应存在");
    assert_eq!(fs::read_to_string(result.join("root.txt")).unwrap(), "root file");

    // 验证二层目录
    assert!(result.join("a/b/file1.txt").exists(), "a/b/file1.txt 应存在");
    assert_eq!(fs::read_to_string(result.join("a/b/file1.txt")).unwrap(), "level 2");

    // 验证三层目录
    assert!(result.join("a/b/c/file2.txt").exists(), "a/b/c/file2.txt 应存在");
    assert_eq!(fs::read_to_string(result.join("a/b/c/file2.txt")).unwrap(), "level 3");

    // 验证另一条深路径
    assert!(result.join("x/y/z/deep.txt").exists(), "x/y/z/deep.txt 应存在");
    assert_eq!(fs::read_to_string(result.join("x/y/z/deep.txt")).unwrap(), "deep nested");
}
