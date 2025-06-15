pub mod parses {
    use clap::ValueEnum;

    pub struct Input {
        pub file: String,
    }

    pub struct Output {
        pub file: Option<String>,
    }

    #[derive(Debug, Clone, ValueEnum)]
    pub enum Format {
        Json,
        Html,
        Plaintext,
        Markdown,
    }

    pub fn parse_options(input: Input, output: Output, format: Format) {
        println!("{}", input.file);
        if let Some(file) = output.file {
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
