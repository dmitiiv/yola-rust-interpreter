mod scanner;
mod source_reader;
mod token;

use std::env;

use scanner::Scanner;
use source_reader::SourceReader;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        println!("THere are more then 1 argument");
    } else if args.len() == 1 {
        println!("There is only one argument");
        run_file(&args[0]);
    } else {
        println!("There is no arguments");
    }
}

fn run_file(path: &str) {
    let mut reader = SourceReader::new();

    if let Err(err) = reader.read_sources(path) {
        eprintln!("Error reading sources: {:?}", err);
        return;
    }

    match reader.get_sources() {
        Ok(_source) => run(_source),
        Err(err) => println!("Error - {:?}", err),
    }
}

fn run(source: String) {
    let scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("Token - {:?}", token);
    }
}
