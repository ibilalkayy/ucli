pub mod validate {
    pub struct ValidateData {
        pub file: String,
    }

    impl ValidateData {
        pub fn validate_options(&self) {
            println!("{}", self.file);
        }
    }
}
