use crate::{
    ast::expression::Expression,
    parser::{self, parser::Parser},
    scanner::{scanner::Scanner, source_reader::SourceReader},
};
use std::io;

pub struct Yola {
    had_error: bool,
}

impl Yola {
    pub fn new() -> Yola {
        Yola { had_error: false }
    }

    pub fn run_file(&mut self, path: &str) {
        let mut reader = SourceReader::new();

        if let Err(err) = reader.read_sources(path) {
            eprintln!("Error reading sources: {:?}", err);
            return;
        }

        match reader.get_sources() {
            Ok(_source) => self.run(_source),
            Err(err) => println!("Error - {:?}", err),
        };
    }

    pub fn run_prompt(&mut self) {
        loop {
            let mut input = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("Error parsing promt");

            let trimmed_input = input.trim();

            if trimmed_input.eq_ignore_ascii_case("exit") {
                break;
            }

            self.run(trimmed_input.to_owned())
        }
    }

    pub fn run(&mut self, source: String) {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens.clone());

        let mut expr: Box<Expression>;
        if let Some(value) = parser.parse() {
            expr = value;
        } else {
            self.set_error();
        }

        if self.had_error {
            return;
        }

        for token in tokens {
            println!("Token - {:?}", token);
        }
    }

    pub fn set_error(&mut self) {
        self.had_error = true;
    }
}
