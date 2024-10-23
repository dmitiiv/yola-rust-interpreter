use crate::lexemes::token::Token;

use super::expr::{Expr, Visitor};

pub struct Binary {
    left: Box<dyn Expr<Parent = Self>>,
    operator: Box<Token>,
    right: Box<dyn Expr<Parent = Self>>,
}

impl Expr for Binary {
    // refer to Self for inheritance purposes
    type Parent = Self;

    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.visit_binary(self)
    }
}
