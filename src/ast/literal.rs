#[derive(Debug)]
pub struct LiteralExpr {
    pub value: String,
}

impl LiteralExpr {
    pub fn new(value: String) -> LiteralExpr {
        LiteralExpr { value }
    }
}
