pub mod cat {
    use std::fs;

    pub struct CatData {
        pub path: Option<String>,
        pub number: bool,
    }

    impl CatData {
        pub fn cat_options(&self) {
            match &self.path {
                Some(file) => {
                    if self.number {
                        let content =
                            fs::read_to_string(file).expect("Err: failed to read the file content");
                        for (index, line) in content.lines().enumerate() {
                            println!("{} {}", index + 1, line);
                        }
                    } else {
                        let content =
                            fs::read_to_string(file).expect("Err: failed to read the file content");
                        println!("{}", content);
                    }
                }
                None => println!("Err: file has not been found"),
            }
        }
    }
}
