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

#[cfg(test)]
mod list_tests {
    use super::list::ListData;
    use std::fs::{self, File};
    use std::path::Path;

    #[test]
    fn test_list_shows_created_file() {
        let dir_name = "list_dir_test";
        let file_name = "file.txt";
        let file_path = format!("{}/{}", dir_name, file_name);

        fs::create_dir_all(dir_name).expect("Failed to create test directory");
        File::create(&file_path).expect("Failed to create test file");

        let list = ListData {
            path: Some(dir_name.to_string()),
            all: false,
            long: false,
        };

        list.list_options();

        assert!(Path::new(&file_path).exists());
    }
}
