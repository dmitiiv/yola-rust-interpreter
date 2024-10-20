use super::expr::Expr;

// #[derive(Debug)]
pub struct Group {
    expression: Box<dyn Expr>,
}

impl Group {
    pub fn new(&self, expression: dyn Expr) -> Group {
        Group {
            expression: Box::new(expression),
        }
    }
}
