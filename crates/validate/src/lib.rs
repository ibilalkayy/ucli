pub mod validate {
    pub struct ValidateFile {
        pub input: String,
    }

    impl ValidateFile {
        pub fn validate_options(&self) {
            println!("{}", self.input);
        }
    }
}
