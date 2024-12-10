use crate::lexemes::token::Token;

use super::expr::{Expr, Visitor};

pub struct Binary<'a, T: 'a> {
    pub left: Box<dyn Expr<T> + 'a>,
    pub operator: Token,
    pub right: Box<dyn Expr<T> + 'a>,
}

impl<'a, T> Binary<'a, T> {
    pub fn new(
        left: Box<dyn Expr<T> + 'a>,
        operator: Token,
        right: Box<dyn Expr<T> + 'a>,
    ) -> Box<dyn Expr<T> + 'a> {
        Box::new(Binary {
            left,
            operator,
            right,
        })
    }
}

impl<'a, T> Expr<T> for Binary<'a, T> {
    fn accept(&self, visitor: &dyn Visitor<T>) -> T {
        visitor.visit_binary(self)
    }
}
