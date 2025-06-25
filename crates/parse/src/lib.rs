pub mod parse {
    pub struct ParseData {
        pub input_file: String,
        pub output_file: Option<String>,
    }

    impl ParseData {
        pub fn parse_options(&self) {
            println!("{}", self.input_file);
            if let Some(file) = &self.output_file {
                println!("{}", file);
            }
        }
    }
}
