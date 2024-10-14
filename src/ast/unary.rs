use crate::lexemes::token::Token;

use super::expr::Expr;

pub struct UnaryExpr {
    operator: Token,
    right: Box<Expr>,
}

impl UnaryExpr {
    pub fn new(&self, operator: Token, right: Expr) -> UnaryExpr {
        UnaryExpr {
            operator,
            right: Box::new(right),
        }
    }
}
