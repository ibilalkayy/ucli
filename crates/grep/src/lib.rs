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
                    for (i, line) in content.lines().enumerate() {
                        let mut matches = line.contains(&self.pattern);

                        if self.case_insensitive {
                            matches = line.to_lowercase().contains(&self.pattern.to_lowercase());
                        }

                        if self.invert {
                            matches = !matches;
                        }

                        if matches {
                            if self.number {
                                println!("{} {}", i + 1, line.trim());
                            } else {
                                println!("{}", line.trim());
                            }
                        } else {
                            eprintln!("Err: pattern has not been matched");
                            return;
                        }
                    }
                }
                Err(error) => eprintln!("Err: {}", error),
            }
        }
    }
}
