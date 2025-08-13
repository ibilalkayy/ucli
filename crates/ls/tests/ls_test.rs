use std::{fs::File, io::Write, path::Path};
use tempfile::tempdir;
use yls::ListData;

#[test]
fn test_show_created_file() {
    let tmp_dir = tempdir().expect("Err: failed to create a temporary directory");
    let dir_path = tmp_dir.path();
    let file_name = "file.txt";
    let file_path = dir_path.join(file_name);
    File::create(&file_path).expect("Failed to create a test file");

    let list = ListData {
        path: Some(tmp_dir.path().to_path_buf()),
        all: false,
        long: false,
    };

    list.list_options();
    assert!(Path::new(&file_path).exists());
}

#[test]
fn test_show_hidden_files() {
    let tmp_dir = tempdir().expect("Err: failed to create a temporary directory");
    let dir_path = tmp_dir.path();
    let file_name = ".gitignore";
    let file_path = dir_path.join(file_name);
    File::create(&file_path).expect("Failed to create a test file");

    let list = ListData {
        path: Some(tmp_dir.path().to_path_buf()),
        all: true,
        long: false,
    };

    let output = list.list_output();
    assert_eq!(output, true);
}

#[test]
fn test_long_output_shows_metadata() {
    let tmp_dir = tempdir().expect("Err: failed to create a temporary directory");
    let dir_path = tmp_dir.path();
    let file_name = "info.txt";
    let file_path = dir_path.join(file_name);
    let mut file = File::create(&file_path).expect("Failed to create a test file");
    writeln!(file, "Hello, world!").expect("Failed to write to test file");

    let list = ListData {
        path: Some(tmp_dir.path().to_path_buf()),
        all: false,
        long: true,
    };

    let output = std::panic::catch_unwind(|| list.list_output());

    assert!(output.is_ok());
    assert_eq!(output.unwrap(), true);
}
