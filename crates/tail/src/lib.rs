pub mod tail {
    pub struct TailData {
        pub path: String,
    }

    impl TailData {
        pub fn tail_options(&self) {
            println!("{}", self.path);
        }
    }
}
