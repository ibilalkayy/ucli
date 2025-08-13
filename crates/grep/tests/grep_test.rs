use std::io::Write;
use tempfile::NamedTempFile;
use ygrep::GrepData;

#[test]
fn test_find_file() {
    let mut file = NamedTempFile::new().expect("Err: failed to create a file");
    write!(file, "search text").expect("Err: failed to write to a file");

    let grep_data = GrepData {
        file: file.path().to_path_buf(),
        pattern: "".to_string(),
        case_insensitive: false,
        invert: false,
        number: false,
    };

    grep_data.grep_options();
}

#[test]
fn test_match_pattern() {
    let mut file = NamedTempFile::new().expect("Err: failed to create a file");
    write!(file, "this is the file where it contains the search text")
        .expect("Err: failed to write to a file");

    let grep_data = GrepData {
        file: file.path().to_path_buf(),
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
}

#[test]
fn test_match_case() {
    let mut file = NamedTempFile::new().expect("Err: failed to create a file");
    write!(file, "this is the file where it contains the search text")
        .expect("Err: failed to write to a file");

    let grep_data = GrepData {
        file: file.path().to_path_buf(),
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
}

#[test]
fn test_match_number() {
    let mut file = NamedTempFile::new().expect("Err: failed to create a file");
    write!(file, "this is the file where it contains the search text")
        .expect("Err: failed to write to a file");

    let grep_data = GrepData {
        file: file.path().to_path_buf(),
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
}

#[test]
fn test_match_invert() {
    let mut file = NamedTempFile::new().expect("Err: failed to create a file");
    writeln!(file, "this line does not match").expect("Err: failed to write to a file");
    writeln!(file, "this is the file where it contains the search text")
        .expect("Err: failed to write to a file");
    writeln!(file, "another unrelated line").expect("Err: failed to write to a file");

    let grep_data = GrepData {
        file: file.path().to_path_buf(),
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
}
