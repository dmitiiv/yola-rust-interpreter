use crate::lexemes::token::Token;

use super::expr::{Expr, Visitor};

pub struct Unary<T> {
    pub operator: Box<Token>,
    pub right: Box<dyn Expr<T> + 'static>,
}

impl<T> Expr<T> for Unary<T> {
    fn accept(&self, visitor: &dyn Visitor<T>) -> T {
        visitor.visit_unary(self)
    }
}
