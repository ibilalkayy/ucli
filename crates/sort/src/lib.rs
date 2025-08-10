use std::{fs, path::PathBuf, u32};

pub struct SortData {
    pub file: Option<PathBuf>,
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
