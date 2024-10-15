use crate::lexemes::token::Token;

use super::expr::Expr;

#[derive(Debug)]
pub struct BinaryExpr {
    left: Box<Expr>,
    operator: Box<Token>,
    right: Box<Expr>,
}

impl BinaryExpr {
    pub fn new(&self, left: Expr, operator: Token, right: Expr) -> BinaryExpr {
        BinaryExpr {
            left: Box::new(left),
            operator: Box::new(operator),
            right: Box::new(right),
        }
    }
}
