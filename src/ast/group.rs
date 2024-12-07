use super::expr::{Expr, Visitor};

pub struct Group<T> {
    expression: Box<dyn Expr<T>>,
}

impl<T> Expr<T> for Group<T> {
    fn accept(&mut self, visitor: &dyn Visitor<&str>) -> T {
        visitor.visit_group(self)
    }
}
