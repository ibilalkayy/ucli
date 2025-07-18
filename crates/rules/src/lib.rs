pub mod rules {
    use std::fs;

    pub fn rules_option() {
        let content = fs::read_to_string("../rules.txt");
        match content {
            Ok(rules) => println!("{}", rules),
            Err(error) => eprintln!("Err: {}", error),
        }
    }
}
