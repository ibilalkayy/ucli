pub mod init {
    pub struct InitData {
        pub your_path: Option<String>,
    }

    impl InitData {
        pub fn init_options(&self) {
            if let Some(path) = &self.your_path {
                println!("Here is the path: {}", path);
            }
        }
    }
}
