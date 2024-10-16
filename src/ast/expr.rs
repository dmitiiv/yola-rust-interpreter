use super::{binary::Binary, literal::Literal, unary::Unary};

#[derive(Debug)]

pub enum Expr {
    BinaryExpr(Binary),
    UnaryExpr(Unary),
    LiteralExpr(Literal),
}
