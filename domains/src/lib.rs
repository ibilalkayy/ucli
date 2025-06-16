pub mod init {
    pub struct InitData {
        pub directory: String,
        pub your_template: String,
    }

    impl InitData {
        pub fn init_options(&self) {
            println!("{}", self.directory);
            println!("{}", self.your_template);
        }
    }
}

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

pub mod validate {
    pub struct ValidateFile {
        pub input: String,
    }

    impl ValidateFile {
        pub fn validate_options(&self) {
            println!("{}", self.input);
        }
    }
}

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
