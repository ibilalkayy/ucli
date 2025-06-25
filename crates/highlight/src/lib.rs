pub mod highlight {
    pub struct HighlightData {
        pub file: String,
        pub theme: Option<String>,
    }

    impl HighlightData {
        pub fn highlight_options(&self) {
            println!("Here is the file: {}", self.file);

            if let Some(ref theme) = self.theme {
                println!("Here is the theme: {}", theme);
            }
        }
    }
}
