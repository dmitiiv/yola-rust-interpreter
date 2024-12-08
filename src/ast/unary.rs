use crate::lexemes::token::Token;

use super::expr::{Expr, Visitor};

pub struct Unary<T> {
    operator: Box<Token>,
    right: Box<dyn Expr<T>>,
}

impl<T> Expr<T> for Unary<T> {
    fn accept(&self, visitor: &dyn Visitor<T>) -> T {
        visitor.visit_unary(self)
    }
}
