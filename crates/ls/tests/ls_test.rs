use std::fs::{self, File};
use std::path::Path;
use yls::ListData;

#[test]
fn test_list_shows_created_file() {
    let dir_name = "list_dir_test1";
    let file_name = "file.txt";
    let file_path = format!("{}/{}", dir_name, file_name);

    fs::create_dir_all(dir_name).expect("Failed to create test directory");
    File::create(&file_path).expect("Failed to create test file");

    let list = ListData {
        path: Some(dir_name.into()),
        all: false,
        long: false,
    };

    list.list_options();

    assert!(Path::new(&file_path).exists());

    fs::remove_file(&file_path).expect("Failed to remove test file");
    fs::remove_dir(dir_name).expect("Failed to remove test directory");
}

#[test]
fn test_show_hidden_files() {
    let dir_name = "list_dir_test2";
    let file_name = ".gitignore";
    let file_path = format!("{}/{}", dir_name, file_name);

    fs::create_dir_all(dir_name).expect("Err: failed to create a directory");
    File::create(&file_path).expect("Err: failed to create a file");

    let list = ListData {
        path: Some(dir_name.into()),
        all: true,
        long: false,
    };

    let output = list.list_output();
    assert_eq!(output, true);

    fs::remove_file(&file_path).expect("Err: failed to remove the file");
    fs::remove_dir(dir_name).expect("Err: failed to remove the directory");
}

#[test]
fn test_long_output_shows_metadata() {
    let dir_name = "list_dir_test3";
    let file_name = "info.txt";
    let file_path = format!("{}/{}", dir_name, file_name);

    // Setup test directory and file
    fs::create_dir_all(dir_name).expect("Failed to create test directory");
    let mut file = File::create(&file_path).expect("Failed to create test file");
    use std::io::Write;
    writeln!(file, "Hello, world!").expect("Failed to write to test file");

    let list = ListData {
        path: Some(dir_name.into()),
        all: false,
        long: true,
    };

    // Capture the output
    let output = std::panic::catch_unwind(|| list.list_output());

    assert!(output.is_ok());
    assert_eq!(output.unwrap(), true);

    // Cleanup
    fs::remove_file(&file_path).expect("Failed to remove test file");
    fs::remove_dir(dir_name).expect("Failed to remove test directory");
}
