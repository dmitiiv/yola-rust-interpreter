use super::expr::{Expr, Visitor};

pub struct Group {
    expression: Box<dyn Expr>,
}

impl Expr for Group {
    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.visit_group(self)
    }
}
