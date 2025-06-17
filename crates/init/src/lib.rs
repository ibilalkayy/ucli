pub mod init {
    pub struct InitData {
        pub directory: String,
        pub your_template: String,
    }

    impl InitData {
        pub fn init_options(&self) {
            println!("{}", self.directory);
            println!("{}", self.your_template);
        }
    }
}
