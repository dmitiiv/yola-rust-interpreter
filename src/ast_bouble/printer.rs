use crate::ast_bouble::expression::{self, Expr, Expression, Visitor};

pub struct AstPrinter {}

impl AstPrinter {
    fn print(&self, mut expr: impl Expr<String>) {
        expr.accept(self);
    }

    fn parenthesize(&self, name: &str, exprs: Vec<&Box<Expression>>) -> String {
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

impl Visitor<String> for AstPrinter {
    fn visit_binary(&self, binary: &crate::ast_bouble::expression::BinaryExp) -> String {
        self.parenthesize(&binary.operator.lexeme, vec![&binary.left, &binary.right])
    }

    fn visit_unary(&self, unary: &crate::ast_bouble::expression::UnaryExp) -> String {
        self.parenthesize(&unary.operator.lexeme, vec![&unary.right])
    }

    fn visit_group(&self, group: &crate::ast_bouble::expression::GroupExp) -> String {
        self.parenthesize("group", vec![&group.expression])
    }

    fn visit_literal(&self, literal: &crate::ast_bouble::expression::LiteralExp) -> String {
        if literal.value.is_empty() {
            return "nil".to_string();
        }

        literal.value.to_owned()
    }
}
