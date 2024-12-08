use crate::lexemes::token::Token;

use super::expr::{Expr, Visitor};

pub struct Unary<'a, T> {
    pub operator: Box<Token>,
    pub right: Box<&'a dyn Expr<T>>,
}

impl<'a, T> Expr<T> for Unary<'a, T> {
    fn accept(&self, visitor: &dyn Visitor<T>) -> T {
        visitor.visit_unary(self)
    }
}
