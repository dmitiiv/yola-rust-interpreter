use super::expr::{Expr, Visitor};

pub struct Group {
    expression: Box<dyn Expr<Parent = Self>>,
}

impl Expr for Group {
    // refer to Self for inheritance purposes
    type Parent = Self;

    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.visit_group(self)
    }
}
