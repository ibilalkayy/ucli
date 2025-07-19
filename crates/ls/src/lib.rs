pub mod list {
    pub struct ListData {
        pub path: String,
    }

    impl ListData {
        pub fn list_options(&self) {
            println!("{}", self.path);
        }
    }
}
