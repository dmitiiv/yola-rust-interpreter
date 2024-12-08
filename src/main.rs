mod ast;
mod ast_printer;
mod lang;
mod lexemes;
mod parser;
mod report;
mod scanner;
mod utils;

use std::env;

use lang::yola::Yola;
use lexemes::token_type::TokenType;

#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

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

lazy_static! {
    static ref KEYWORDS: HashMap<String, TokenType> = {
        let mut keywords = HashMap::new();
        keywords.insert("and".to_string(), TokenType::AND);
        keywords.insert("class".to_string(), TokenType::CLASS);
        keywords.insert("else".to_string(), TokenType::ELSE);
        keywords.insert("false".to_string(), TokenType::FALSE);
        keywords.insert("for".to_string(), TokenType::FOR);
        keywords.insert("func".to_string(), TokenType::FUNC);
        keywords.insert("if".to_string(), TokenType::IF);
        keywords.insert("nil".to_string(), TokenType::NIL);
        keywords.insert("or".to_string(), TokenType::OR);
        keywords.insert("print".to_string(), TokenType::PRINT);
        keywords.insert("return".to_string(), TokenType::RETURN);
        keywords.insert("super".to_string(), TokenType::SUPER);
        keywords.insert("this".to_string(), TokenType::THIS);
        keywords.insert("true".to_string(), TokenType::TRUE);
        keywords.insert("var".to_string(), TokenType::VAR);
        keywords.insert("loop".to_string(), TokenType::LOOP);

        keywords
    };
}
