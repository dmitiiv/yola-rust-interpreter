use super::{binary::BinaryExpr, literal::LiteralExpr, unary::UnaryExpr};

#[derive(Debug)]

pub enum Expr {
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    Literal(LiteralExpr),
}
