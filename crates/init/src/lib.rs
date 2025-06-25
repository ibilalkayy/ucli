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
