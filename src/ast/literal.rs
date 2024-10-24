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
    // refer to Self for inheritance purposes
    type Parent = Self;

    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.visit_literal(self)
    }
}
