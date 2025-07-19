pub mod sort {
    pub struct SortData {
        pub file: String,
    }

    impl SortData {
        pub fn sort_options(&self) {
            println!("Here is the file: {}", self.file);
        }
    }
}
