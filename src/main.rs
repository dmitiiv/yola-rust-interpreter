mod scanner;
mod source_reader;
use std::env;

use scanner::Scanner;
use source_reader::SourceReader;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        println!("args len {:?}", &args)
    } else if args.len() == 1 {
        println!("args len {:?}", &args);
        run_file(&args[0]);
    } else {
        println!("args len {:?}", &args)
    }
}

fn run_file(path: &str) {
    let mut reader = SourceReader::new();

    reader
        .read_sources(path)
        .expect("Error during read sources");

    match reader.get_sources() {
        Ok(_source) => run(_source),
        Err(err) => println!("Error - {:?}", err),
    }
}

fn run(source: String) {
    let scanner = Scanner::new(source);
}
