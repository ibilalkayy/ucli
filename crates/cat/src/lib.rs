pub mod cat {
    pub struct CatData {
        pub path: Option<String>,
        pub number: Option<u32>,
    }

    impl CatData {
        pub fn cat_options(&self) {
            println!("{} {}", self.path.is_some(), self.number.is_some());
        }
    }
}
