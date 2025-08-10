use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};
use ysort::SortData;

#[test]
fn test_read_file() {
    let file_name = PathBuf::from("test1.txt");
    let content = "1 Line\n2 Line\n3 Line";

    let mut file = File::create(&file_name).expect("Err: failed to create a file");
    file.write_all(content.as_bytes())
        .expect("Err: failed to write to a file");

    let data = SortData {
        file: Some(file_name.to_path_buf()),
        reverse: false,
        number: false,
    };

    data.sort_options();

    let read_content = fs::read_to_string(&file_name).expect("Err: failed to read the file");
    assert_eq!(read_content, content);

    fs::remove_file(file_name).expect("Err: failed to remove the file");
}

#[test]
fn test_trim_data() {
    let file_name = PathBuf::from("test2.txt");
    let content = "1 Line\n\n2 Line\n3 Line\n\n4 Line";

    let mut file = File::create(&file_name).expect("Err: failed to create a file");
    file.write_all(content.as_bytes())
        .expect("Err: failed to write to a file");

    let data = SortData {
        file: Some(file_name.to_path_buf()),
        reverse: false,
        number: false,
    };

    data.sort_options();

    let read_content = fs::read_to_string(&file_name).expect("Err: failed to read the file");
    let lines: Vec<&str> = read_content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();

    let expected_lines: Vec<&str> = content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();

    assert_eq!(lines, expected_lines);

    fs::remove_file(file_name).expect("Err: failed to remove the file");
}

#[test]
fn test_sort_by_number() {
    let file_name = PathBuf::from("test4.txt");
    let content = "1 Line\n\n2 Line\n3 Line\n\n4 Line";

    let mut file = File::create(&file_name).expect("Err: failed to create a file");
    file.write_all(content.as_bytes())
        .expect("Err: failed to write to a file");

    let data = SortData {
        file: Some(file_name.to_path_buf()),
        reverse: false,
        number: true,
    };

    let sorted_lines = data.sort_output();

    assert_eq!(sorted_lines, vec!["1 Line", "2 Line", "3 Line", "4 Line"]);

    fs::remove_file(file_name).expect("Err: failed to remove the file");
}

#[test]
fn test_sort_by_reverse() {
    let file_name = PathBuf::from("test5.txt");
    let content = "1 Line\n\n2 Line\n3 Line\n\n4 Line";

    let mut file = File::create(&file_name).expect("Err: failed to create a file");
    file.write_all(content.as_bytes())
        .expect("Err: failed to write to a file");

    let data = SortData {
        file: Some(file_name.to_path_buf()),
        reverse: true,
        number: false,
    };

    let sorted_lines = data.sort_output();

    assert_eq!(sorted_lines, vec!["4 Line", "3 Line", "2 Line", "1 Line"]);

    fs::remove_file(file_name).expect("Err: failed to remove the file");
}
