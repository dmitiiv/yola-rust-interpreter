use crate::lexemes::token::Token;

use super::expr::{Expr, Expressions, Visitor};

pub struct Binary {
    left: Box<dyn Expr<Parent = Expressions>>,
    operator: Box<Token>,
    right: Box<dyn Expr<Parent = Expressions>>,
}

impl Expr for Binary {
    // refer to Self for inheritance purposes
    type Parent = Self;

    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.visit_binary(self)
    }
}
