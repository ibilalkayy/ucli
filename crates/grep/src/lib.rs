pub mod grep {
    use std::fs;

    pub struct GrepData {
        pub file: String,
        pub pattern: String,
        pub case_insensitive: bool,
        pub invert: bool,
        pub number: bool,
    }

    impl GrepData {
        pub fn grep_output(&self) -> Vec<String> {
            let mut results = vec![];
            match fs::read_to_string(&self.file) {
                Ok(content) => {
                    let mut found = false;

                    for (i, line) in content.lines().enumerate() {
                        let mut matches = if self.case_insensitive {
                            line.to_lowercase().contains(&self.pattern.to_lowercase())
                        } else {
                            line.contains(&self.pattern)
                        };

                        if self.invert {
                            matches = !matches;
                        }

                        if matches {
                            found = true;
                            if self.number {
                                results.push(format!("{} {}", i + 1, line.trim()));
                            } else {
                                results.push(line.trim().to_string());
                            }
                        }
                    }

                    if !found {
                        eprintln!("Err: pattern has not been matched");
                    }
                }
                Err(error) => eprintln!("Err: {}", error),
            }
            results
        }

        pub fn grep_options(&self) {
            let _ = self.grep_output();
        }
    }
}

#[cfg(test)]
mod grep_tests {
    use std::{fs::File, io::Write, path::PathBuf};

    use crate::grep::GrepData;

    #[test]
    fn test_find_file() {
        let file_path = PathBuf::from("grep_file.txt");

        let mut file = File::create(&file_path).expect("Err: failed to create a file");
        file.write_all("search text".as_bytes())
            .expect("Err: failed to write to a file");

        let grep_data = GrepData {
            file: file_path.to_str().unwrap().to_string(),
            pattern: "".to_string(),
            case_insensitive: false,
            invert: false,
            number: false,
        };

        grep_data.grep_options();
    }

    #[test]
    fn test_match_pattern() {
        let file_path = PathBuf::from("grep_pattern.txt");

        let mut file = File::create(&file_path).expect("Err: failed to create a file");
        file.write_all("this is the file where it contains the search text".as_bytes())
            .expect("Err: failed to write to a file");

        let grep_data = GrepData {
            file: file_path.to_str().unwrap().to_string(),
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
        let file_path = PathBuf::from("grep_case.txt");

        let mut file = File::create(&file_path).expect("Err: failed to create a file");
        file.write_all("this is the file where it contains the search text".as_bytes())
            .expect("Err: failed to write to a file");

        let grep_data = GrepData {
            file: file_path.to_str().unwrap().to_string(),
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
        let file_path = PathBuf::from("grep_number.txt");

        let mut file = File::create(&file_path).expect("Err: failed to create a file");
        file.write_all("this is the file where it contains the search text".as_bytes())
            .expect("Err: failed to write to a file");

        let grep_data = GrepData {
            file: file_path.to_str().unwrap().to_string(),
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
        let file_path = PathBuf::from("grep_invert.txt");

        let mut file = File::create(&file_path).expect("Err: failed to create a file");
        writeln!(file, "this line does not match").unwrap();
        writeln!(file, "this is the file where it contains the search text").unwrap();
        writeln!(file, "another unrelated line").unwrap();

        let grep_data = GrepData {
            file: file_path.to_str().unwrap().to_string(),
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
}
