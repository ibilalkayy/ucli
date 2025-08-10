use std::{fs, path::PathBuf};

pub struct CatData {
    pub path: Option<PathBuf>,
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
