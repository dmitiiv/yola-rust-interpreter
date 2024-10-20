// use super::{binary::Binary, literal::Literal, unary::Unary};

// #[derive(Debug)]

pub enum Expressions {
    BinaryExpr(Binary),
    UnaryExpr(Unary),
    // LiteralExpr(Literal),
}

use crate::lexemes::token::Token;

trait Visitor {
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

trait Expr {
    // need for inheritance
    type Parent: Expr;

    // visitor pattern
    fn accept(&mut self, visitor: &mut dyn Visitor);
}

pub struct Binary {
    left: Box<dyn Expr<Parent = Expressions>>,
    operator: Box<Token>,
    right: Box<dyn Expr<Parent = Expressions>>,
}

impl Expr for Binary {
    // refer to Self for inheritance purposes
    type Parent = Self;

    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.binary_visitor(self)
    }
}

pub struct Unary {
    operator: Box<Token>,
    right: Box<dyn Expr<Parent = Expressions>>,
}

impl Expr for Unary {
    // refer to Self for inheritance purposes
    type Parent = Self;

    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.unary_visitor(self)
    }
}
