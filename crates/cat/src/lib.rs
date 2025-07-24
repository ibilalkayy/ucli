pub mod cat {
    use std::fs;

    pub struct CatData {
        pub path: Option<String>,
        pub number: bool,
    }

    impl CatData {
        pub fn cat_output(&self) -> String {
            match &self.path {
                Some(file) => {
                    let content =
                        fs::read_to_string(file).expect("Err: failed to read the file content");
                    if self.number {
                        content
                            .lines()
                            .enumerate()
                            .map(|(x, line)| format!("{} {}", x + 1, line))
                            .collect::<Vec<_>>()
                            .join("\n")
                    } else {
                        content
                    }
                }
                None => "Err: file has not been found".to_string(),
            }
        }

        pub fn cat_options(&self) {
            let output = self.cat_output();
            println!("{}", output);
        }
    }
}

#[cfg(test)]
mod cat_tests {
    use crate::cat::CatData;
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;

    #[test]
    fn test_read_file_num_false() {
        let file_path = PathBuf::from("test_file.txt");

        let mut file = File::create(&file_path).expect("Err: failed to create a file");
        file.write_all("Line 1\nLine 2".as_bytes())
            .expect("Err: failed to write to test file");

        let cat_data = CatData {
            path: Some(file_path.to_str().unwrap().to_string()),
            number: false,
        };

        let output = cat_data.cat_output();
        assert_eq!(output, "Line 1\nLine 2")
    }

    #[test]
    fn test_read_file_num_true() {
        let file_path = PathBuf::from("test_file.txt");

        let mut file = File::create(&file_path).expect("Err: failed to create a file");
        file.write_all("Line 1\nLine 2".as_bytes())
            .expect("Err: failed to write to test file");

        let cat_data = CatData {
            path: Some(file_path.to_str().unwrap().to_string()),
            number: true,
        };

        let output = cat_data.cat_output();
        assert_eq!(output, "1 Line 1\n2 Line 2")
    }
}
