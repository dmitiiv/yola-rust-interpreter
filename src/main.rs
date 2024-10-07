mod lang;
mod lexemes;
mod report;
mod scanner;
mod utils;

use std::env;

use lang::yola::Yola;

fn main() {
    let args: Vec<String> = env::args().collect();

    let yola = Yola::new();

    if args.len() > 2 {
        println!("THere are more then 1 argument");
    } else if args.len() == 2 {
        println!("There is only one argument");
        yola.run_file(&args[1]);
    } else {
        println!("There is no arguments");
        yola.run_prompt();
    }
}
