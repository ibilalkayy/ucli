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
        pub fn grep_options(&self) {
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
                                println!("{} {}", i + 1, line.trim());
                            } else {
                                println!("{}", line.trim());
                            }
                        }
                    }

                    if !found {
                        eprintln!("Err: pattern has not been matched");
                    }
                }
                Err(error) => eprintln!("Err: {}", error),
            }
        }
    }
}
