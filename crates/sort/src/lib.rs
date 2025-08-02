pub mod sort {
    use std::{fs, u32};

    pub struct SortData {
        pub file: Option<String>,
        pub reverse: bool,
        pub number: bool,
    }

    impl SortData {
        pub fn sort_options(&self) {
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

                        for line in lines {
                            println!("{}", line);
                        }
                    }
                    Err(error) => eprintln!("Err: {}", error),
                },
                None => eprintln!("Err: file does not exist"),
            }
        }
    }
}
