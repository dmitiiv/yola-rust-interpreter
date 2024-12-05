use crate::lexemes::token::Token;

use super::expr::{Expr, Visitor};

pub struct Binary {
    left: Box<dyn Expr>,
    operator: Box<Token>,
    right: Box<dyn Expr>,
}

impl Expr for Binary {
    fn accept(&mut self, visitor: &dyn Visitor) {
        visitor.visit_binary(self)
    }
}
