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

impl Expr for Literal {
    fn accept(&mut self, visitor: &dyn Visitor) {
        visitor.visit_literal(self)
    }
}
