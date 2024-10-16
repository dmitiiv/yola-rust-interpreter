use crate::lexemes::token::Token;

use super::expr::Expr;

#[derive(Debug)]
pub struct Unary {
    operator: Box<Token>,
    right: Box<Expr>,
}

impl Unary {
    pub fn new(&self, operator: Token, right: Expr) -> Unary {
        Unary {
            operator: Box::new(operator),
            right: Box::new(right),
        }
    }
}
