pub mod watch {
    use clap::ValueEnum;

    pub struct WatchFiles {
        pub path: String,
        pub on_change: String,
        pub output: Option<String>,
    }

    #[derive(Debug, Clone, ValueEnum)]
    pub enum Format {
        Json,
        Html,
        Plaintext,
        Markdown,
    }

    impl WatchFiles {
        pub fn watch_options(&self, format: Format) {
            println!("{}", self.path);
            if let Some(ref file) = self.output {
                println!("{}", file);
            }
            println!("{}", self.on_change);

            match format {
                Format::Json => println!("Json is selected"),
                Format::Html => println!("HTML is selected"),
                Format::Plaintext => println!("Plaintext is selected"),
                Format::Markdown => println!("Markdown is selected"),
            }
        }
    }
}
