use super::{binary::Binary, group::Group, unary::Unary};

pub enum Expressions {
    BinaryExpr(Binary),
    UnaryExpr(Unary),
    LiteralExpr(Group),
}

pub trait Visitor {
    fn visit_binary(&mut self, expr: &Binary);
    fn visit_unary(&mut self, expr: &Unary);
    fn visit_group(&mut self, expr: &Group);
}

pub struct ExprVisitor {}

impl Visitor for ExprVisitor {
    fn visit_binary(&mut self, expr: &Binary) {
        println!("Binary visitor")
    }

    fn visit_unary(&mut self, expr: &Unary) {
        println!("Unary visitor")
    }

    fn visit_group(&mut self, expr: &Group) {
        println!("Unary visitor")
    }
}

pub trait Expr {
    // need for inheritance
    type Parent: Expr;

    // visitor pattern
    fn accept(&mut self, visitor: &mut dyn Visitor);
}
