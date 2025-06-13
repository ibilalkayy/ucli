pub mod parses {

    pub struct Output {
        pub file: String,
    }

    pub struct Format {
        pub json: bool,
        pub html: bool,
        pub plaintext: bool,
        pub markdown: bool,
    }

    pub struct Template {
        pub template_type: String,
    }

    pub fn parse_options(output: Output, format: Format, template: Template) {
        println!("{}", output.file);
        match format {
            Format { json: true, .. } => println!("Json is selected"),
            Format { html: true, .. } => println!("Html is selected"),
            Format {
                plaintext: true, ..
            } => println!("Plaintext is selected"),
            Format { markdown: true, .. } => println!("Markdown is selected"),
            _ => println!("nothing is selected"),
        }
        println!("{}", template.template_type);
    }
}
