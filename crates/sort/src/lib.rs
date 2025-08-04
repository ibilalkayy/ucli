pub mod sort {
    use std::{fs, u32};

    pub struct SortData {
        pub file: Option<String>,
        pub reverse: bool,
        pub number: bool,
    }

    impl SortData {
        pub fn sort_output(&self) -> Vec<String> {
            match &self.file {
                Some(file) => match fs::read_to_string(file) {
                    Ok(content) => {
                        let mut lines: Vec<&str> = content
                            .lines()
                            .filter(|line| !line.trim().is_empty())
                            .collect();

                        if self.number {
                            lines.sort_by_key(|line| {
                                line.split_whitespace()
                                    .next()
                                    .and_then(|s| s.parse::<u32>().ok())
                                    .unwrap_or(u32::MAX)
                            });
                        } else {
                            lines.sort();
                        }

                        if self.reverse {
                            lines.reverse();
                        }

                        lines.into_iter().map(|s| s.to_string()).collect()
                    }
                    Err(error) => {
                        eprintln!("Err: {}", error);
                        vec![]
                    }
                },
                None => {
                    eprintln!("Err: file does not exist");
                    vec![]
                }
            }
        }

        pub fn sort_options(&self) {
            let _ = self.sort_output();
        }
    }
}

#[cfg(test)]
mod sort_tests {
    use super::sort::SortData;
    use std::{
        fs::{self, File},
        io::Write,
        path::PathBuf,
    };

    #[test]
    fn test_read_file() {
        let file_name = PathBuf::from("test1.txt");
        let content = "1 Line\n2 Line\n3 Line";

        let mut file = File::create(&file_name).expect("Err: failed to create a file");
        file.write_all(content.as_bytes())
            .expect("Err: failed to write to a file");

        let data = SortData {
            file: Some(file_name.to_str().unwrap().to_string()),
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
            file: Some(file_name.to_str().unwrap().to_string()),
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
            file: Some(file_name.to_str().unwrap().to_string()),
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
            file: Some(file_name.to_str().unwrap().to_string()),
            reverse: true,
            number: false,
        };

        let sorted_lines = data.sort_output();

        assert_eq!(sorted_lines, vec!["4 Line", "3 Line", "2 Line", "1 Line"]);

        fs::remove_file(file_name).expect("Err: failed to remove the file");
    }
}
