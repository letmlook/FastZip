//! 集成测试

use std::fs;
use std::io::Write;

use fastzip_core::{detect_format, extract_one, ArchiveFormat, ExtractOptions};
use tempfile::TempDir;
use zip::write::SimpleFileOptions;

fn create_test_zip() -> (TempDir, std::path::PathBuf) {
    let dir = TempDir::new().unwrap();
    let zip_path = dir.path().join("test.zip");

    let mut zip = zip::ZipWriter::new(std::fs::File::create(&zip_path).unwrap());
    zip.start_file("hello.txt", SimpleFileOptions::default()).unwrap();
    zip.write_all(b"Hello World").unwrap();
    zip.finish().unwrap();

    (dir, zip_path)
}

#[test]
fn test_detect_zip_format() {
    let (_dir, zip_path) = create_test_zip();
    let format = detect_format(&zip_path).unwrap();
    assert_eq!(format, ArchiveFormat::Zip);
}

#[test]
fn test_extract_zip_smart() {
    let (dir, zip_path) = create_test_zip();
    let dest_dir = dir.path().join("out");

    let options = ExtractOptions {
        dest: Some(dest_dir.clone()),
        smart: true,
        overwrite: false,
        password: None,
    };

    let result = extract_one(&zip_path, &options).unwrap();
    assert!(result.join("hello.txt").exists());
    let content = fs::read_to_string(result.join("hello.txt")).unwrap();
    assert_eq!(content, "Hello World");
}

#[test]
fn test_extract_zip_single_file_smart_dest() {
    let (dir, zip_path) = create_test_zip();
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
    assert!(dest_dir.join("hello.txt").exists());
}
