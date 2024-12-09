use super::expr::{Expr, Visitor};

pub struct Group<'a, T> {
    pub expression: Box<&'a dyn Expr<T>>,
}

impl<'a, T> Expr<T> for Group<'a, T> {
    fn accept(&self, visitor: &dyn Visitor<T>) -> T {
        visitor.visit_group(self)
    }
}
