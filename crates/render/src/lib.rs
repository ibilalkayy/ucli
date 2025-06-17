pub mod render {
    use clap::ValueEnum;

    pub struct RenderFiles {
        pub input_file: String,
        pub output_file: Option<String>,
        pub template_data: String,
    }

    #[derive(Debug, Clone, ValueEnum)]
    pub enum Format {
        Json,
        Html,
        Plaintext,
        Markdown,
    }

    impl RenderFiles {
        pub fn render_options(&self, format: Format) {
            println!("{}", self.input_file);
            if let Some(ref file) = self.output_file {
                println!("{}", file);
            }
            println!("{}", self.template_data);

            match format {
                Format::Json => println!("Json is selected"),
                Format::Html => println!("HTML is selected"),
                Format::Plaintext => println!("Plaintext is selected"),
                Format::Markdown => println!("Markdown is selected"),
            }
        }
    }
}
