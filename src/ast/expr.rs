use crate::lexemes::token::Token;

use super::{binary::Binary, group::Group, literal::Literal, unary::Unary};
pub trait Visitor<T> {
    fn visit_binary(&self, expr: &Binary<T>) -> T;
    fn visit_unary(&self, expr: &Unary<T>) -> T;
    fn visit_group(&self, expr: &Group<T>) -> T;
    fn visit_literal(&self, expr: &Literal) -> T;
}

pub struct ExprVisitor {}

impl Visitor<()> for ExprVisitor {
    fn visit_binary(&self, expr: &Binary<()>) {
        println!("Binary visitor");
    }

    fn visit_unary(&self, expr: &Unary<()>) {
        println!("Unary visitor");
    }

    fn visit_group(&self, expr: &Group<()>) {
        println!("Unary visitor");
    }

    fn visit_literal(&self, expr: &Literal) {
        println!("Unary literal");
    }
}

pub trait Expr<T> {
    // visitor pattern
    fn accept(&self, visitor: &dyn Visitor<T>) -> T;
}
