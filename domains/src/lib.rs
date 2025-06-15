pub mod init {
    pub struct InitData {
        pub directory: String,
        pub your_template: String,
    }

    pub fn init_options(data: InitData) {
        println!("{}", data.directory);
        println!("{}", data.your_template);
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

    pub fn parse_options(file_type: ParseFiles, format: Format) {
        println!("{}", file_type.input_file);
        if let Some(file) = file_type.output_file {
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

    pub fn render_options(file_type: RenderFiles, format: Format) {
        println!("{}", file_type.input_file);
        if let Some(file) = file_type.output_file {
            println!("{}", file);
        }
        println!("{}", file_type.template_data);

        match format {
            Format::Json => println!("Json is selected"),
            Format::Html => println!("HTML is selected"),
            Format::Plaintext => println!("Plaintext is selected"),
            Format::Markdown => println!("Markdown is selected"),
        }
    }
}

pub mod validate {
    pub struct ValidateFile {
        pub input: String,
    }

    pub fn validate_options(file_type: ValidateFile) {
        println!("{}", file_type.input);
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

    pub fn watch_options(watch_file: WatchFiles, format: Format) {
        println!("{}", watch_file.path);
        if let Some(file) = watch_file.output {
            println!("{}", file);
        }
        println!("{}", watch_file.on_change);

        match format {
            Format::Json => println!("Json is selected"),
            Format::Html => println!("HTML is selected"),
            Format::Plaintext => println!("Plaintext is selected"),
            Format::Markdown => println!("Markdown is selected"),
        }
    }
}
pub mod lint {
    pub struct LintData {
        pub input: String,
    }

    pub fn lint_options(data: LintData) {
        println!("{}", data.input);
    }
}
