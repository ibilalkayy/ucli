use std::{fs, io::Write};

use tempfile::NamedTempFile;
use ywc::WcData;

#[test]
fn test_read_file() {
    let content = "1 Line\n2 Line\n3 Line";
    let mut file = NamedTempFile::new().expect("Err: failed to create a file");
    write!(file, "{}", content).expect("Err: failed to write to a file");

    let data = WcData {
        file: Some(file.path().to_path_buf()),
        lines: false,
        words: false,
        bytes: false,
    };

    data.wc_options();
    let read_content = fs::read_to_string(&file).expect("Err: failed to read the file");
    assert_eq!(read_content, content);
}

#[test]
fn test_count_lines() {
    let content = "1 Line\n2 Line\n3 Line";
    let mut file = NamedTempFile::new().expect("Err: failed to create a file");
    write!(file, "{}", content).expect("Err: failed to write to a file");

    let data = WcData {
        file: Some(file.path().to_path_buf()),
        lines: true,
        words: false,
        bytes: false,
    };

    data.wc_options();
    let read_content = fs::read_to_string(&file).expect("Err: failed to read the file");
    let lines = read_content.lines().count();
    assert_eq!(lines, 3);
}

#[test]
fn test_count_words() {
    let content = "Line1 Line2 Line3";
    let mut file = NamedTempFile::new().expect("Err: failed to create a file");
    write!(file, "{}", content).expect("Err: failed to write to a file");

    let data = WcData {
        file: Some(file.path().to_path_buf()),
        lines: false,
        words: true,
        bytes: false,
    };

    data.wc_options();

    let read_content = fs::read_to_string(&file).expect("Err: failed to read the file");
    let words = read_content.split(' ').enumerate().last();
    let mut result: usize = 0;
    match words {
        Some((index, _)) => {
            result = index + 1;
        }
        None => println!("Err: no value found"),
    }
    assert_eq!(result, 3);
}

#[test]
fn test_count_bytes() {
    let content = "1 Line";
    let mut file = NamedTempFile::new().expect("Err: failed to create a file");
    write!(file, "{}", content).expect("Err: failed to write to a file");

    let data = WcData {
        file: Some(file.path().to_path_buf()),
        lines: false,
        words: false,
        bytes: true,
    };

    data.wc_options();
    let read_content = fs::read_to_string(&file).expect("Err: failed to read the file");
    let length = read_content.len();
    assert_eq!(length, 6);
}
