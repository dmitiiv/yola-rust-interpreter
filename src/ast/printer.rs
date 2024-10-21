use super::expr::Expr;

pub struct Printer {}

impl Printer {
    fn print(&self, expr: Expr) {
        expr.visitor(&self);
    }
}
