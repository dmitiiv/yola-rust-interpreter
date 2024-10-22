use crate::ast::expr::{Expr, Expressions, Visitor};

pub struct AstPrinter {}

impl AstPrinter {
    fn print(&mut self, expr: Box<&dyn Expr<Parent = Expressions>>) {
        expr.accept(self);
    }
}

// impl Expr for AstPrinter {
//     type Parent = Self;

//     fn accept(&mut self, visitor: &mut dyn Visitor) {
//         todo!()
//     }
// }

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
}
