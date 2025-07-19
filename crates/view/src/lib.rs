pub mod view {
    pub struct ViewData {
        pub file: String,
    }

    impl ViewData {
        pub fn view_options(&self) {
            println!("File: {}", self.file);
        }
    }
}
