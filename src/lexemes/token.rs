use super::token_type::TokenType;
use crate::ast::expression::LiteralExp;

#[derive(Debug, Clone)]
pub struct Token {
    pub id: TokenType,
    pub lexeme: String,
    literal: Option<LiteralExp>,
    line: usize,
}

impl Token {
    pub fn new(id: TokenType, lexeme: String, line: usize, literal: Option<LiteralExp>) -> Token {
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
