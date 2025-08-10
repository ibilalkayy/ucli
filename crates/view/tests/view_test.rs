use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};
use yview::ViewData;

#[test]
fn test_read_file() {
    let file_name = PathBuf::from("test.txt");
    let content = "Line1\nLine2\nLine3";

    let mut file = File::create(&file_name).expect("Err: failed to create a file");
    file.write_all(content.as_bytes())
        .expect("Err: failed to write to a file");

    let view = ViewData {
        file: Some(file_name.to_path_buf()),
        lines: 2,
    };

    view.view_options();

    let read_content = fs::read_to_string(&file_name).expect("Err: failed to read the file");
    assert_eq!(read_content, content);

    fs::remove_file(file_name).expect("Err: failed to remove the file");
}
