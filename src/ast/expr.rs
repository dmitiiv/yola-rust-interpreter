use super::{binary::Binary, literal::Literal, unary::Unary};

#[derive(Debug)]

pub enum Expr {
    BinaryExpr(Binary),
    UnaryExpr(Unary),
    LiteralExpr(Literal),
}

#[derive(Debug)]
pub struct Group {
    expression: Box<Expr>,
}

impl Group {
    pub fn new(&self, expression: Expr) -> Group {
        Group {
            expression: Box::new(expression),
        }
    }
}
