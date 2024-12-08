use crate::ast::{
    binary::Binary,
    expr::{self, Expr, Visitor},
};

pub struct AstPrinter {}

impl AstPrinter {
    fn print(&self, mut expr: impl Expr<String>) {
        expr.accept(self);
    }

    fn parenthesize(&self, name: &str, exprs: Vec<Box<&dyn Expr<String>>>) -> String {
        let mut builder = String::new();

        builder.push('(');
        builder.push_str(name);

        for expr in exprs {
            builder.push(' ');
            builder.push_str(&expr.accept(self));
        }

        builder
    }
}

impl<'a> Visitor<String> for AstPrinter {
    fn visit_binary(&self, binary: &Binary<String>) -> String {
        self.parenthesize(
            &binary.operator.lexeme,
            vec![binary.left.clone(), binary.right.clone()],
        )
    }

    fn visit_unary(&self, expr: &crate::ast::unary::Unary<String>) -> String {
        String::new()
    }

    fn visit_group(&self, expr: &crate::ast::group::Group<String>) -> String {
        String::new()
    }

    fn visit_literal(&self, expr: &crate::ast::literal::Literal) -> String {
        String::new()
    }
}
