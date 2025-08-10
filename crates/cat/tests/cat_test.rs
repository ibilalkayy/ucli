use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use ycat::CatData;

#[test]
fn test_read_file_num_false() {
    let file_path = PathBuf::from("cat_file.txt");

    let mut file = File::create(&file_path).expect("Err: failed to create a file");
    file.write_all("Line 1\nLine 2".as_bytes())
        .expect("Err: failed to write to test file");

    let cat_data = CatData {
        path: Some(file_path.to_path_buf()),
        number: false,
    };

    let output = cat_data.cat_output();
    assert_eq!(output, "Line 1\nLine 2");

    fs::remove_file(&file_path).expect("Err: failed to remove a file");
}

#[test]
fn test_read_file_num_true() {
    let file_path = PathBuf::from("test_file.txt");

    let mut file = File::create(&file_path).expect("Err: failed to create a file");
    file.write_all("Line 1\nLine 2".as_bytes())
        .expect("Err: failed to write to test file");

    let cat_data = CatData {
        path: Some(file_path.to_path_buf()),
        number: true,
    };

    let output = cat_data.cat_output();
    assert_eq!(output, "1 Line 1\n2 Line 2");

    fs::remove_file(&file_path).expect("Err: failed to remove a file");
}
