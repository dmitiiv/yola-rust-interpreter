use super::{binary::Binary, group::Group, literal::Literal, unary::Unary};

// pub enum Expressions {
//     BinaryExpr(Binary),
//     UnaryExpr(Unary),
//     LiteralExpr(Group),
// }

pub trait Visitor<T> {
    fn visit_binary(&self, expr: &Binary<T>) -> T;
    fn visit_unary(&self, expr: &Unary<T>) -> T;
    fn visit_group(&self, expr: &Group<T>) -> T;
    fn visit_literal(&self, expr: &Literal) -> T;
}

pub struct ExprVisitor {}

impl Visitor<&str> for ExprVisitor {
    fn visit_binary(&self, expr: &Binary<&str>) -> &'static str {
        println!("Binary visitor");
        "binary"
    }

    fn visit_unary(&self, expr: &Unary<&str>) -> &'static str {
        println!("Unary visitor");
        "unary"
    }

    fn visit_group(&self, expr: &Group<&str>) -> &'static str {
        println!("Unary visitor");
        "group"
    }

    fn visit_literal(&self, expr: &Literal) -> &'static str {
        println!("Unary literal");
        "literal"
    }
}

pub trait Expr<T> {
    // visitor pattern
    fn accept(&mut self, visitor: &dyn Visitor<&'static str>) -> T;
}
