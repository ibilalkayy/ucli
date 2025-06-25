pub mod example {
    pub struct ExampleData {
        pub path: String,
    }

    impl ExampleData {
        pub fn example_options(&self) {
            println!("{}", self.path);
        }
    }
}
