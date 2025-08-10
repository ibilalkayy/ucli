use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};
use ygrep::GrepData;

#[test]
fn test_find_file() {
    let file_path = PathBuf::from("grep_file.txt");

    let mut file = File::create(&file_path).expect("Err: failed to create a file");
    file.write_all("search text".as_bytes())
        .expect("Err: failed to write to a file");

    let grep_data = GrepData {
        file: file_path.to_path_buf(),
        pattern: "".to_string(),
        case_insensitive: false,
        invert: false,
        number: false,
    };

    grep_data.grep_options();

    fs::remove_file(&file_path).expect("Err: failed to remove a file");
}

#[test]
fn test_match_pattern() {
    let file_path = PathBuf::from("grep_pattern.txt");

    let mut file = File::create(&file_path).expect("Err: failed to create a file");
    file.write_all("this is the file where it contains the search text".as_bytes())
        .expect("Err: failed to write to a file");

    let grep_data = GrepData {
        file: file_path.to_path_buf(),
        pattern: "search".to_string(),
        case_insensitive: false,
        invert: false,
        number: false,
    };

    let output = grep_data.grep_output();
    assert_eq!(
        output,
        vec!["this is the file where it contains the search text"]
    );

    fs::remove_file(&file_path).expect("Err: failed to remove a file");
}

#[test]
fn test_match_case() {
    let file_path = PathBuf::from("grep_case.txt");

    let mut file = File::create(&file_path).expect("Err: failed to create a file");
    file.write_all("this is the file where it contains the search text".as_bytes())
        .expect("Err: failed to write to a file");

    let grep_data = GrepData {
        file: file_path.to_path_buf(),
        pattern: "Search".to_string(),
        case_insensitive: true,
        invert: false,
        number: false,
    };

    let output = grep_data.grep_output();
    assert_eq!(
        output,
        vec!["this is the file where it contains the search text"]
    );

    fs::remove_file(&file_path).expect("Err: failed to remove a file");
}

#[test]
fn test_match_number() {
    let file_path = PathBuf::from("grep_number.txt");

    let mut file = File::create(&file_path).expect("Err: failed to create a file");
    file.write_all("this is the file where it contains the search text".as_bytes())
        .expect("Err: failed to write to a file");

    let grep_data = GrepData {
        file: file_path.to_path_buf(),
        pattern: "search".to_string(),
        case_insensitive: false,
        invert: false,
        number: true,
    };

    let output = grep_data.grep_output();
    assert_eq!(
        output,
        vec!["1 this is the file where it contains the search text"]
    );

    fs::remove_file(&file_path).expect("Err: failed to remove a file");
}

#[test]
fn test_match_invert() {
    let file_path = PathBuf::from("grep_invert.txt");

    let mut file = File::create(&file_path).expect("Err: failed to create a file");
    writeln!(file, "this line does not match").unwrap();
    writeln!(file, "this is the file where it contains the search text").unwrap();
    writeln!(file, "another unrelated line").unwrap();

    let grep_data = GrepData {
        file: file_path.to_path_buf(),
        pattern: "search".to_string(),
        case_insensitive: false,
        invert: true,
        number: false,
    };

    let output = grep_data.grep_output();
    assert_eq!(
        output,
        vec!["this line does not match", "another unrelated line"]
    );

    fs::remove_file(&file_path).expect("Err: failed to remove a file");
}
