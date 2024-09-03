use std::borrow::Borrow;

use super::{
    literal::{self, Literal},
    token_type::TokenType,
};

#[derive(Debug)]
pub struct Token {
    id: TokenType,
    lexeme: String,
    literal: Option<Literal>,
    line: usize,
}

impl Token {
    pub fn new(id: TokenType, lexeme: String, line: usize, literal: Option<Literal>) -> Token {
        Token {
            id,
            lexeme,
            literal,
            line,
        }
    }

    pub fn toString(&self) -> String {
        self.line.to_string() + &self.lexeme // + self.literal
    }
}
