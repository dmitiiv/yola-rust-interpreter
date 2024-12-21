use crate::ast::expression::Visitor;

pub struct Interpretier {}

impl Visitor<String> for Interpretier {
    fn visit_binary(&self, expr: &crate::ast::expression::BinaryExp) -> () {
        todo!()
    }

    fn visit_unary(&self, expr: &crate::ast::expression::UnaryExp) -> () {
        todo!()
    }

    fn visit_group(&self, expr: &crate::ast::expression::GroupExp) -> () {
        todo!()
    }

    fn visit_literal(&self, expr: &crate::ast::expression::LiteralExp) -> String {
        expr.value.clone()
    }
}
