pub mod lint {
    pub struct LintData {
        pub input: String,
    }

    impl LintData {
        pub fn lint_options(&self) {
            println!("{}", self.input);
        }
    }
}
