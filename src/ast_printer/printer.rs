use crate::ast::expr::{Expr, Expressions, Visitor};

pub struct AstPrinter {}

impl AstPrinter {
    fn print(&self, mut expr: impl Expr) {
        expr.accept(self);
    }

    fn parenthesize(name: &str, exprs: Vec<Box<dyn Expr>>) {}
}

impl Visitor for AstPrinter {
    fn visit_binary(&self, expr: &crate::ast::binary::Binary) {
        todo!()
    }

    fn visit_unary(&self, expr: &crate::ast::unary::Unary) {
        todo!()
    }

    fn visit_group(&self, expr: &crate::ast::group::Group) {
        todo!()
    }

    fn visit_literal(&self, expr: &crate::ast::literal::Literal) {
        todo!()
    }
}
