pub mod view {
    use std::fs;

    pub struct ViewData {
        pub file: Option<String>,
        pub lines: usize,
    }

    impl ViewData {
        pub fn view_options(&self) {
            match &self.file {
                Some(file) => match fs::read_to_string(file) {
                    Ok(content) => {
                        let lines = content.lines().take(self.lines);
                        for line in lines {
                            println!("{}", line);
                        }
                    }
                    Err(error) => eprintln!("Err: {}", error),
                },
                None => eprintln!("Err: file has been selected"),
            }
        }
    }
}

#[cfg(test)]
mod view_tests {
    use super::view::ViewData;
    use std::{
        fs::{self, File},
        io::Write,
        path::PathBuf,
    };

    #[test]
    fn test_read_file() {
        let file_name = PathBuf::from("test.txt");
        let content = "Line1\nLine2\nLine3";

        let mut file = File::create(&file_name).expect("Err: failed to create a file");
        file.write_all(content.as_bytes())
            .expect("Err: failed to write to a file");

        let view = ViewData {
            file: Some(file_name.to_str().unwrap().to_string()),
            lines: 2,
        };

        view.view_options();

        let read_content = fs::read_to_string(&file_name).expect("Failed to read file");
        assert_eq!(read_content, content);

        fs::remove_file(file_name).expect("Err: failed to remove the file");
    }
}
