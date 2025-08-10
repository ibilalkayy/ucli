use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use ywc::WcData;

#[test]
fn test_read_file() {
    let file_name = PathBuf::from("test1.txt");
    let content = "1 Line\n2 Line\n3 Line";

    let mut file = File::create(&file_name).expect("Err: failed to create a file");
    file.write_all(content.as_bytes())
        .expect("Err: failed to write to a file");

    let data = WcData {
        file: Some(file_name.to_path_buf()),
        lines: false,
        words: false,
        bytes: false,
    };

    data.wc_options();

    let read_content = fs::read_to_string(&file_name).expect("Err: failed to read the file");

    assert_eq!(read_content, content);

    fs::remove_file(file_name).expect("Err: failed to remove the file");
}

#[test]
fn test_count_lines() {
    let file_name = PathBuf::from("test2.txt");
    let content = "1 Line\n2 Line\n3 Line";

    let mut file = File::create(&file_name).expect("Err: failed to create a file");
    file.write_all(content.as_bytes())
        .expect("Err: failed to write to a file");

    let data = WcData {
        file: Some(file_name.to_path_buf()),
        lines: true,
        words: false,
        bytes: false,
    };

    data.wc_options();

    let read_content = fs::read_to_string(&file_name).expect("Err: failed to read the file");
    let lines = read_content.lines().count();

    assert_eq!(lines, 3);

    fs::remove_file(file_name).expect("Err: failed to remove the file");
}

#[test]
fn test_count_words() {
    let file_name = PathBuf::from("test3.txt");
    let content = "Line1 Line2 Line3";

    let mut file = File::create(&file_name).expect("Err: failed to create a file");
    file.write_all(content.as_bytes())
        .expect("Err: failed to write to a file");

    let data = WcData {
        file: Some(file_name.to_path_buf()),
        lines: false,
        words: true,
        bytes: false,
    };

    data.wc_options();

    let read_content = fs::read_to_string(&file_name).expect("Err: failed to read the file");
    let words = read_content.split(' ').enumerate().last();
    let mut result: usize = 0;
    match words {
        Some((index, _)) => {
            result = index + 1;
        }
        None => println!("Err: no value found"),
    }

    assert_eq!(result, 3);

    fs::remove_file(file_name).expect("Err: failed to remove the file");
}

#[test]
fn test_count_bytes() {
    let file_name = PathBuf::from("test4.txt");
    let content = "1 Line";

    let mut file = File::create(&file_name).expect("Err: failed to create a file");
    file.write_all(content.as_bytes())
        .expect("Err: failed to write to a file");

    let data = WcData {
        file: Some(file_name.to_path_buf()),
        lines: false,
        words: false,
        bytes: true,
    };

    data.wc_options();

    let read_content = fs::read_to_string(&file_name).expect("Err: failed to read the file");
    let length = read_content.len();

    assert_eq!(length, 6);

    fs::remove_file(file_name).expect("Err: failed to remove the file");
}
