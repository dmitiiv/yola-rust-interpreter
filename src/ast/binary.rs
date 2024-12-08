use crate::lexemes::token::Token;

use super::expr::{Expr, Visitor};

pub struct Binary<'a, T> {
    pub left: Box<&'a dyn Expr<T>>,
    pub operator: Token,
    pub right: Box<&'a dyn Expr<T>>,
}

impl<'a, T> Expr<T> for Binary<'a, T> {
    fn accept(&self, visitor: &dyn Visitor<T>) -> T {
        visitor.visit_binary(self)
    }
}
