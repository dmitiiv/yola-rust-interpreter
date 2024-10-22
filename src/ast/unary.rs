use crate::lexemes::token::Token;

use super::expr::{Expr, Visitor};

pub struct Unary {
    operator: Box<Token>,
    right: Box<dyn Expr<Parent = Self>>,
}

impl Expr for Unary {
    // refer to Self for inheritance purposes
    type Parent = Self;

    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.visit_unary(self)
    }
}
