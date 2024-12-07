use crate::lexemes::token::Token;

use super::expr::{Expr, Visitor};

pub struct Unary<T> {
    operator: Box<Token>,
    right: Box<dyn Expr<T>>,
}

impl<T> Expr<T> for Unary<T> {
    fn accept(&mut self, visitor: &dyn Visitor<&str>) -> T {
        visitor.visit_unary(self)
    }
}
