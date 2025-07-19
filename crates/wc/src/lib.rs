pub mod wc {
    pub struct WcData {
        pub file: String,
    }

    impl WcData {
        pub fn wc_options(&self) {
            println!("File: {}", self.file);
        }
    }
}
