use std::{fs, path::PathBuf};

pub struct ViewData {
    pub file: Option<PathBuf>,
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
            None => eprintln!("Err: file has not been selected"),
        }
    }
}
