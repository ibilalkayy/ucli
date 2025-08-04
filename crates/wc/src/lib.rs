pub mod wc {
    use std::fs;

    pub struct WcData {
        pub file: Option<String>,
        pub lines: bool,
        pub words: bool,
        pub bytes: bool,
    }

    impl WcData {
        pub fn wc_options(&self) {
            match &self.file {
                Some(file) => match fs::read_to_string(file) {
                    Ok(content) => {
                        if self.lines {
                            let lines = content.lines().count();
                            println!("Number of total lines: {}", lines);
                        } else if self.words {
                            let words = content.split(' ').enumerate().last();
                            match words {
                                Some((index, _)) => {
                                    println!("Number of total words: {}", index + 1)
                                }
                                None => println!("Err: no value found"),
                            }
                        } else if self.bytes {
                            println!("Number of total bytes: {}", content.len());
                        } else {
                            eprintln!("Err: select anyone flag");
                        }
                    }
                    Err(error) => println!("Err: {}", error),
                },
                None => eprintln!("Err: file does not exist"),
            }
        }
    }
}
