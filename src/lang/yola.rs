use crate::scanner::{scanner::Scanner, source_reader::SourceReader};
use std::io;

pub struct Yola {
    had_error: bool,
}

impl Yola {
    pub fn new() -> Yola {
        Yola { had_error: false }
    }

    pub fn run_file(&self, path: &str) {
        let mut reader = SourceReader::new();

        if let Err(err) = reader.read_sources(path) {
            eprintln!("Error reading sources: {:?}", err);
            return;
        }

        match reader.get_sources() {
            Ok(_source) => Yola::run(_source),
            Err(err) => println!("Error - {:?}", err),
        };
    }

    pub fn run_prompt(&self) {
        loop {
            let mut input = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("Error parsing promt");

            let trimmed_input = input.trim();

            if trimmed_input.eq_ignore_ascii_case("exit") {
                break;
            }

            Yola::run(trimmed_input.to_owned())
        }
    }

    pub fn run(source: String) {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        for token in tokens {
            println!("Token - {:?}", token);
        }
    }

    pub fn set_error(&mut self) {
        self.had_error = true;
    }
}
