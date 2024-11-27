use crate::lexemes::token::Token;

use super::expr::{Expr, Visitor};

pub struct Unary {
    operator: Box<Token>,
    right: Box<dyn Expr>,
}

impl Expr for Unary {
    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.visit_unary(self)
    }
}
