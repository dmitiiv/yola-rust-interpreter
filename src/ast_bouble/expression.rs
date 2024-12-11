use crate::lexemes::token::Token;

pub enum Expression {
    Binary(Box<BinaryExp>),
    Unary(Box<UnaryExp>),
    Group(Box<GroupExp>),
}
pub struct BinaryExp {
    pub left: Box<Expression>,
    pub operator: Token,
    pub right: Box<Expression>,
}

impl BinaryExp {
    pub fn new(left: Box<Expression>, operator: Token, right: Box<Expression>) -> BinaryExp {
        BinaryExp {
            left,
            operator,
            right,
        }
    }
}

impl<T> Expr<T> for BinaryExp {
    fn accept(&self, visitor: &dyn Visitor<T>) -> T {
        visitor.visit_binary(self)
    }
}

pub struct UnaryExp {
    pub operator: Token,
    pub right: Box<Expression>,
}

impl UnaryExp {
    pub fn new(operator: Token, right: Box<Expression>) -> UnaryExp {
        UnaryExp { operator, right }
    }
}

impl<T> Expr<T> for UnaryExp {
    fn accept(&self, visitor: &dyn Visitor<T>) -> T {
        visitor.visit_unary(self)
    }
}

pub struct GroupExp {
    pub expression: Box<Expression>,
}

impl GroupExp {
    pub fn new(expression: Box<Expression>) -> GroupExp {
        GroupExp { expression }
    }
}

impl<T> Expr<T> for GroupExp {
    fn accept(&self, visitor: &dyn Visitor<T>) -> T {
        visitor.visit_group(self)
    }
}

pub struct LiteralExp {
    pub value: String,
}

impl LiteralExp {
    pub fn new(value: String) -> LiteralExp {
        LiteralExp { value }
    }
}

impl<T> Expr<T> for LiteralExp {
    fn accept(&self, visitor: &dyn Visitor<T>) -> T {
        visitor.visit_literal(self)
    }
}
pub trait Visitor<T> {
    fn visit_binary(&self, expr: &BinaryExp) -> T;
    fn visit_unary(&self, expr: &UnaryExp) -> T;
    fn visit_group(&self, expr: &GroupExp) -> T;
    fn visit_literal(&self, expr: &LiteralExp) -> T;
}

pub trait Expr<T> {
    // visitor pattern
    fn accept(&self, visitor: &dyn Visitor<T>) -> T;
}
