pub mod cat {
    use std::fs;

    pub struct CatData {
        pub path: Option<String>,
        pub number: Option<u32>,
    }

    impl CatData {
        pub fn cat_options(&self) {
            match &self.number {
                Some(add_num) => println!("{}", add_num),
                None => match &self.path {
                    Some(file) => {
                        let content =
                            fs::read_to_string(file).expect("Err: failed to read the file content");
                        for (index, line) in content.lines().enumerate() {
                            println!("{} {}", index + 1, line);
                        }
                    }
                    None => println!("Err: file has not been found"),
                },
            }
        }
    }
}
