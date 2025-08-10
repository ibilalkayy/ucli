use std::{fs, path::PathBuf};

pub struct GrepData {
    pub file: PathBuf,
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
