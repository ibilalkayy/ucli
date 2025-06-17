pub mod parse {
    use clap::ValueEnum;

    pub struct ParseFiles {
        pub input_file: String,
        pub output_file: Option<String>,
    }

    #[derive(Debug, Clone, ValueEnum)]
    pub enum Format {
        Json,
        Html,
        Plaintext,
        Markdown,
    }

    impl ParseFiles {
        pub fn parse_options(&self, format: Format) {
            println!("{}", self.input_file);
            if let Some(ref file) = self.output_file {
                println!("{}", file);
            }

            match format {
                Format::Json => println!("Json is selected"),
                Format::Html => println!("HTML is selected"),
                Format::Plaintext => println!("Plaintext is selected"),
                Format::Markdown => println!("Markdown is selected"),
            }
        }
    }
}
