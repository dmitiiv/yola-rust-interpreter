use crate::ast::expr::{Expr, Visitor};

pub struct AstPrinter {}

impl AstPrinter {
    fn print(&self, mut expr: impl Expr<&'static str>) {
        expr.accept(self);
    }

    fn parenthesize(&self, name: &str, exprs: Vec<Box<dyn Expr<&'static str>>>) -> String {
        let mut builder = String::new();

        builder.push('(');
        builder.push_str(name);

        for expr in exprs {
            builder.push(' ');
            builder.push_str(&expr.accept(self));
            // builder.push('!');
        }

        builder
    }
}

impl Visitor<&str> for AstPrinter {
    fn visit_binary(&self, expr: &crate::ast::binary::Binary<&str>) -> &'static str {
        " "
    }

    fn visit_unary(&self, expr: &crate::ast::unary::Unary<&str>) -> &'static str {
        ""
    }

    fn visit_group(&self, expr: &crate::ast::group::Group<&str>) -> &'static str {
        ""
    }

    fn visit_literal(&self, expr: &crate::ast::literal::Literal) -> &'static str {
        ""
    }
}
