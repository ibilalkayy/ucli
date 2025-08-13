use std::fs;
use std::io::Write;
use tempfile::NamedTempFile;
use ysort::SortData;

#[test]
fn test_read_file() {
    let content = "1 Line\n2 Line\n3 Line";
    let mut file = NamedTempFile::new().expect("Err: failed to create a file");
    write!(file, "{}", content).expect("Err: failed to write to a file");

    let data = SortData {
        file: Some(file.path().to_path_buf()),
        reverse: false,
        number: false,
    };

    data.sort_options();

    let read_content = fs::read_to_string(&file).expect("Err: failed to read a file");
    assert_eq!(read_content, content);
}

#[test]
fn test_trim_data() {
    let content = "1 Line\n\n2 Line\n3 Line\n\n4 Line";
    let mut file = NamedTempFile::new().expect("Err: failed to create a file");
    write!(file, "{}", content).expect("Err: failed to write to a file");

    let data = SortData {
        file: Some(file.path().to_path_buf()),
        reverse: false,
        number: false,
    };

    data.sort_options();

    let read_content = fs::read_to_string(&file).expect("Err: failed to read the file");
    let lines: Vec<&str> = read_content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();

    let expected_lines: Vec<&str> = content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();

    assert_eq!(lines, expected_lines);
}

#[test]
fn test_sort_by_number() {
    let content = "1 Line\n\n2 Line\n3 Line\n\n4 Line";
    let mut file = NamedTempFile::new().expect("Err: failed to create a file");
    write!(file, "{}", content).expect("Err: failed to write to a file");

    let data = SortData {
        file: Some(file.path().to_path_buf()),
        reverse: false,
        number: true,
    };

    let sorted_lines = data.sort_output();

    assert_eq!(sorted_lines, vec!["1 Line", "2 Line", "3 Line", "4 Line"]);
}

#[test]
fn test_sort_by_reverse() {
    let content = "1 Line\n\n2 Line\n3 Line\n\n4 Line";
    let mut file = NamedTempFile::new().expect("Err: failed to create a file");
    write!(file, "{}", content).expect("Err: failed to write to a file");

    let data = SortData {
        file: Some(file.path().to_path_buf()),
        reverse: true,
        number: false,
    };

    let sorted_lines = data.sort_output();

    assert_eq!(sorted_lines, vec!["4 Line", "3 Line", "2 Line", "1 Line"]);
}
