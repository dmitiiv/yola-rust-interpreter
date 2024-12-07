use crate::lexemes::token::Token;

use super::expr::{Expr, Visitor};

pub struct Binary<T> {
    left: Box<dyn Expr<T>>,
    operator: Box<Token>,
    right: Box<dyn Expr<T>>,
}

impl<T> Expr<T> for Binary<T> {
    fn accept(&mut self, visitor: &dyn Visitor<&'static str>) -> T {
        visitor.visit_binary(self)
    }
}
