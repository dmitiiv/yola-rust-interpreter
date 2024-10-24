use super::{binary::Binary, group::Group, literal::Literal, unary::Unary};

pub enum Expressions {
    BinaryExpr(Binary),
    UnaryExpr(Unary),
    LiteralExpr(Group),
}

pub trait Visitor {
    fn visit_binary(&mut self, expr: &Binary);
    fn visit_unary(&mut self, expr: &Unary);
    fn visit_group(&mut self, expr: &Group);
    fn visit_literal(&mut self, expr: &Literal);
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

    fn visit_literal(&mut self, expr: &Literal) {
        todo!();
    }
}

pub trait Expr {
    // need for inheritance
    type Parent: Expr;

    // visitor pattern
    fn accept(&mut self, visitor: &mut dyn Visitor);
}
