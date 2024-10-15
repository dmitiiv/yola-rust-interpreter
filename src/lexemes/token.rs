use std::borrow::Borrow;

use crate::ast::{expr::Expr, literal::LiteralExpr};

use super::token_type::TokenType;

#[derive(Debug)]
pub struct Token {
    pub id: TokenType,
    pub lexeme: String,
    literal: Option<LiteralExpr>,
    line: usize,
}

impl Token {
    pub fn new(id: TokenType, lexeme: String, line: usize, literal: Option<LiteralExpr>) -> Token {
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
