use crate::ast::expr::{Expr, Expressions, Visitor};

pub struct AstPrinter {}

impl AstPrinter {
    fn print(&self, mut expr: impl Expr) {
        expr.accept(self);
    }

    fn parenthesize(name: &str, exprs: Vec<Box<dyn Expr>>) -> String {
        let mut builder = String::new();

        builder.push('(');
        builder.push_str(name);

        for expr in exprs {
            builder.push(' ');
            // builder.push_str(&expr.accept(self));
            builder.push('!');
        }

        builder
    }
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
