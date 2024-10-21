pub enum Expressions {
    BinaryExpr(Binary),
    UnaryExpr(Unary),
    // LiteralExpr(Literal),
}

use super::{binary::Binary, unary::Unary};

pub trait Visitor {
    fn visit_binary(&mut self, expr: Binary);
    fn visit_unary(&mut self, expr: Unary);
}

pub struct ExprVisitor {}

impl ExprVisitor {
    fn binary_visitor(binary: Binary) {
        println!("Binary visitor")
    }

    fn unary_visitor(binary: Binary) {
        println!("Unary visitor")
    }
}

pub trait Expr {
    // need for inheritance
    type Parent: Expr;

    // visitor pattern
    fn accept(&mut self, visitor: &mut dyn Visitor);
}
