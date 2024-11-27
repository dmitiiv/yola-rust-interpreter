use crate::ast::expr::Visitor;

pub struct AstPrinter {}

impl Visitor for AstPrinter {
    fn visit_binary(&mut self, expr: &crate::ast::binary::Binary) {
        todo!()
    }

    fn visit_unary(&mut self, expr: &crate::ast::unary::Unary) {
        todo!()
    }

    fn visit_group(&mut self, expr: &crate::ast::group::Group) {
        todo!()
    }

    fn visit_literal(&mut self, expr: &crate::ast::literal::Literal) {
        todo!()
    }
}
