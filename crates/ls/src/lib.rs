pub mod list {
    use std::fs;

    pub struct ListData {
        pub path: Option<String>,
        pub all: bool,
        pub long: bool,
    }

    impl ListData {
        pub fn list_options(&self) {
            match &self.path {
                Some(dir_path) => {
                    let paths =
                        fs::read_dir(dir_path).expect("Err: failed to read the directory path");

                    for p in paths {
                        let entry = p.unwrap();
                        let file_name = entry.file_name();
                        let name_str = file_name.to_string_lossy();

                        // ✅ Skip hidden files unless `--all` is selected
                        if !self.all && name_str.starts_with('.') {
                            continue;
                        }

                        if self.long {
                            let metadata = entry.metadata().unwrap();
                            let file_type = if metadata.is_dir() { "dir" } else { "file" };
                            let size = if metadata.is_file() {
                                metadata.len().to_string()
                            } else {
                                "-".to_string()
                            };

                            println!("{:<8} {:>8}  {}", file_type, size, name_str);
                        } else {
                            // Normal short mode
                            print!("{:<15}", name_str);
                        }
                    }

                    if !self.long {
                        println!(); // add newline if in short format
                    }
                }
                None => eprintln!("Err: file path has not been provided"),
            }
        }
    }
}
