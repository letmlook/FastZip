//! 多格式解压测试

use std::fs::File;
use std::io::Write;

use fastzip_core::{detect_format, extract_one, ArchiveFormat, ExtractOptions};
use tempfile::TempDir;

fn create_tar_gz() -> (TempDir, std::path::PathBuf) {
    let dir = TempDir::new().unwrap();
    let hello_path = dir.path().join("hello.txt");
    let tar_gz_path = dir.path().join("data.tar.gz");

    File::create(&hello_path).unwrap().write_all(b"Hello World").unwrap();

    let tar_gz_file = File::create(&tar_gz_path).unwrap();
    let enc = flate2::write::GzEncoder::new(tar_gz_file, flate2::Compression::default());
    let mut tar = tar::Builder::new(enc);
    tar.append_path_with_name(&hello_path, "hello.txt").unwrap();
    tar.finish().unwrap();

    (dir, tar_gz_path)
}

#[test]
fn test_tar_gz_format_detection() {
    let (_dir, path) = create_tar_gz();
    let format = detect_format(&path).unwrap();
    assert_eq!(format, ArchiveFormat::TarGz, "应为 TarGz");
}

#[test]
fn test_tar_gz_extract() {
    let (_dir, path) = create_tar_gz();
    let dest = dir.path().join("out");

    let options = ExtractOptions {
        dest: Some(dest.clone()),
        smart: true,
        overwrite: false,
        password: None,
    };

    let result = extract_one(&path, &options).unwrap();
    assert_eq!(result, dest);
    assert!(dest.join("hello.txt").exists());
    let content = std::fs::read_to_string(dest.join("hello.txt")).unwrap();
    assert_eq!(content, "Hello World");
}
