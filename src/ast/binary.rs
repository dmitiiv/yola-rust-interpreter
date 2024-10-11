use crate::lexemes::token::Token;

// pub trait Expr {}
pub struct Expr {}

pub struct Binary {}

impl Binary {
    fn new(&self, left: Expr, operator: Token, right: Expr) -> Binary {
        Binary {}
    }
}
