use crate::lexemes::token::Token;

use super::expr::Expr;

pub struct BinaryExpr {
    left: Box<Expr>,
    operator: Token,
    right: Box<Expr>,
}

impl BinaryExpr {
    pub fn new(&self, left: Expr, operator: Token, right: Expr) -> BinaryExpr {
        BinaryExpr {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
}
