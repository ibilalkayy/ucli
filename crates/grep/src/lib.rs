pub mod grep {
    pub struct GrepData {
        pub case_insensitive: bool,
        pub invert: bool,
        pub numbers: bool,
    }

    impl GrepData {
        pub fn grep_options(&self) {
            println!("{}", self.case_insensitive);
            println!("{}", self.invert);
            println!("{}", self.numbers);
        }
    }
}
