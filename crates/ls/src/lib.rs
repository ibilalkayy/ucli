use std::{fs, path::PathBuf};

pub struct ListData {
    pub path: Option<PathBuf>,
    pub all: bool,
    pub long: bool,
}

impl ListData {
    pub fn list_output(&self) -> bool {
        match &self.path {
            Some(dir_path) => {
                let paths = match fs::read_dir(dir_path) {
                    Ok(p) => p,
                    Err(_) => {
                        eprintln!("Err: failed to read the directory path");
                        return false;
                    }
                };

                for p in paths {
                    let entry = match p {
                        Ok(e) => e,
                        Err(_) => continue,
                    };

                    let file_name = entry.file_name();
                    let name_str = file_name.to_string_lossy();

                    // Skip hidden files unless `--all` is selected
                    if !self.all && name_str.starts_with('.') {
                        continue;
                    }

                    if self.long {
                        let metadata = match entry.metadata() {
                            Ok(m) => m,
                            Err(_) => continue,
                        };

                        let file_type = if metadata.is_dir() { "dir" } else { "file" };
                        let size = if metadata.is_file() {
                            metadata.len().to_string()
                        } else {
                            "-".to_string()
                        };

                        println!("{:<8} {:>8}  {}", file_type, size, name_str);
                    } else {
                        print!("{:<15}", name_str);
                    }
                }

                if !self.long {
                    println!();
                }

                true
            }
            None => {
                eprintln!("Err: file path has not been provided");
                false
            }
        }
    }

    pub fn list_options(&self) {
        let _ = self.list_output();
    }
}
