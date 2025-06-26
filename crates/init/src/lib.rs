pub mod init {
    pub struct InitData {
        pub your_path: Option<String>,
        pub force: bool,
    }

    use std::{fs::OpenOptions, io::Write};

    impl InitData {
        pub fn init_options(&self) {
            match &self.your_path {
                Some(path) => {
                    if self.force {
                        let mut file = OpenOptions::new()
                            .create(true)
                            .truncate(true)
                            .write(true)
                            .open(path)
                            .unwrap();
                        file.write_all(b"world")
                            .expect("Err: failed to create a overwrite a file");
                        println!("File is successfully overwritten");
                    } else {
                        let mut file = OpenOptions::new()
                            .create_new(true)
                            .write(true)
                            .open(path)
                            .expect("Err: file already exists");
                        file.write_all(b"Hello World")
                            .expect("Err: failed to write the file");

                        println!("File is successfully written");
                    }
                }
                None => eprintln!("Err: No path is given"),
            }
        }
    }
}

#[cfg(test)]
mod init_tests {
    use crate::init::InitData;
    use std::fs;

    #[test]
    fn test_create_file_force_false() {
        let path = "first.txt".to_string();

        let _ = fs::remove_file(&path);

        let data = InitData {
            your_path: Some(path.clone()),
            force: false,
        };

        data.init_options();

        assert!(std::path::Path::new(&path).exists());

        let content = fs::read_to_string(&path).unwrap();
        assert_eq!(content, "Hello World");

        fs::remove_file(&path).unwrap();
    }

    #[test]
    fn test_create_file_force_true() {
        let path = "file.txt".to_string();

        fs::write(&path, "Hello bye").unwrap();

        let data = InitData {
            your_path: Some(path.clone()),
            force: true,
        };

        data.init_options();

        let content = fs::read_to_string(&path).unwrap();
        assert_eq!(content, "world");

        fs::remove_file(&path).unwrap();
    }
}
