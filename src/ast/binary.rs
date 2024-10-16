use crate::lexemes::token::Token;

use super::expr::Expr;

#[derive(Debug)]
pub struct Binary {
    left: Box<Expr>,
    operator: Box<Token>,
    right: Box<Expr>,
}

impl Binary {
    pub fn new(&self, left: Expr, operator: Token, right: Expr) -> Binary {
        Binary {
            left: Box::new(left),
            operator: Box::new(operator),
            right: Box::new(right),
        }
    }
}
