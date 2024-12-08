use super::expr::{Expr, Visitor};

#[derive(Debug)]
pub struct Literal {
    value: String,
}

impl Literal {
    pub fn new(value: String) -> Literal {
        Literal { value }
    }
}

impl<T> Expr<T> for Literal {
    fn accept(&self, visitor: &dyn Visitor<T>) -> T {
        visitor.visit_literal(self)
    }
}
