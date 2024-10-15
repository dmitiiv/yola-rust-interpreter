use crate::lexemes::token::Token;

use super::expr::Expr;

#[derive(Debug)]
pub struct UnaryExpr {
    operator: Box<Token>,
    right: Box<Expr>,
}

impl UnaryExpr {
    pub fn new(&self, operator: Token, right: Expr) -> UnaryExpr {
        UnaryExpr {
            operator: Box::new(operator),
            right: Box::new(right),
        }
    }
}
