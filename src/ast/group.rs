use super::expr::Expr;

#[derive(Debug)]
pub struct Group {
    expression: Box<Expr>,
}

impl Group {
    pub fn new(&self, expression: Expr) -> Group {
        Group {
            expression: Box::new(expression),
        }
    }
}
