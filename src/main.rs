mod lang;
mod lexemes;
mod report;
mod scanner;
mod source_reader;

use std::env;

use lang::yola::Yola;

fn main() {
    let args: Vec<String> = env::args().collect();

    let yola = Yola::new();

    if args.len() > 1 {
        println!("THere are more then 1 argument");
    } else if args.len() == 1 {
        println!("There is only one argument");
        yola.run_file(&args[0]);
    } else {
        println!("There is no arguments");
        yola.run_prompt();
    }
}
