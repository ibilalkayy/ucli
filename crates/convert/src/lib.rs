pub mod convert {
    use clap::ValueEnum;

    pub struct ConvertData {
        pub file: String,
        pub output: Option<String>,
    }

    #[derive(Debug, Clone, ValueEnum)]
    pub enum TargetFormat {
        Json,
        Html,
        Toml,
        Plaintext,
        Markdown,
    }

    impl ConvertData {
        pub fn convert_options(&self, format: TargetFormat) {
            println!("Input file: {}", self.file);

            match format {
                TargetFormat::Json => println!("JSON is selected"),
                TargetFormat::Html => println!("HTML is selected"),
                TargetFormat::Toml => println!("TOML is selected"),
                TargetFormat::Plaintext => println!("Plaintext is selected"),
                TargetFormat::Markdown => println!("Markdown is selected"),
            }

            if let Some(output) = &self.output {
                println!("Here is the output file: {}", output);
            }
        }
    }
}
