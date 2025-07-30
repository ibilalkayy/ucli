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
