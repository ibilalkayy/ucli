use std::{fs, io::Write};
use tempfile::NamedTempFile;
use yview::ViewData;

#[test]
fn test_read_file() {
    let content = "Line1\nLine2\nLine3";
    let mut file = NamedTempFile::new().expect("Err: failed to create a file");
    write!(file, "{}", content).expect("Err: failed to write to a file");

    let view = ViewData {
        file: Some(file.path().to_path_buf()),
        lines: 2,
    };

    view.view_options();

    let read_content = fs::read_to_string(&file).expect("Err: failed to read the file");
    assert_eq!(read_content, content);
}
