use super::{binary::Binary, group::Group, literal::Literal, unary::Unary};

pub enum Expressions {
    BinaryExpr(Binary),
    UnaryExpr(Unary),
    LiteralExpr(Group),
}

pub trait Visitor {
    fn visit_binary(&self, expr: &Binary);
    fn visit_unary(&self, expr: &Unary);
    fn visit_group(&self, expr: &Group);
    fn visit_literal(&self, expr: &Literal);
}

pub struct ExprVisitor {}

impl Visitor for ExprVisitor {
    fn visit_binary(&self, expr: &Binary) {
        println!("Binary visitor")
    }

    fn visit_unary(&self, expr: &Unary) {
        println!("Unary visitor")
    }

    fn visit_group(&self, expr: &Group) {
        println!("Unary visitor")
    }

    fn visit_literal(&self, expr: &Literal) {
        todo!();
    }
}

pub trait Expr {
    // visitor pattern
    fn accept(&mut self, visitor: &dyn Visitor);
}
