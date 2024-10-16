use crate::ast::literal::Literal;

use super::token_type::TokenType;

#[derive(Debug)]
pub struct Token {
    pub id: TokenType,
    pub lexeme: String,
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

    pub fn to_string(&self) -> String {
        self.line.to_string() + &self.lexeme // + self.literal
    }
}
