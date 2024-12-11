use super::expr::{Expr, Visitor};

pub struct Group<'a, T: 'a> {
    pub expression: Box<dyn Expr<T> + 'a>,
}

impl<'a, T> Group<'a, T> {
    pub fn new(expression: Box<dyn Expr<T> + 'a>) -> Box<dyn Expr<T> + 'a> {
        Box::new(Group { expression })
    }
}

impl<'a, T> Expr<T> for Group<'a, T> {
    fn accept(&self, visitor: &dyn Visitor<T>) -> T {
        visitor.visit_group(self)
    }
}
