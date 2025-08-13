use std::io::Write;
use tempfile::NamedTempFile;
use ycat::CatData;

#[test]
fn test_read_file_num_false() {
    let mut file = NamedTempFile::new().expect("Err: failed to create a file");
    write!(file, "Line 1\nLine 2").expect("Err: failed to write to a file");

    let cat_data = CatData {
        path: Some(file.path().to_path_buf()),
        number: false,
    };

    let output = cat_data.cat_output();
    assert_eq!(output, "Line 1\nLine 2");
}

#[test]
fn test_read_file_num_true() {
    let mut file = NamedTempFile::new().expect("Err: failed to create a file");
    write!(file, "Line 1\nLine 2").expect("Err: failed to write to a file");

    let cat_data = CatData {
        path: Some(file.path().to_path_buf()),
        number: true,
    };

    let output = cat_data.cat_output();
    assert_eq!(output, "1 Line 1\n2 Line 2");
}
