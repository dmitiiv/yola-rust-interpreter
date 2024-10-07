#[derive(Debug)]
pub struct Literal {
    pub value: String,
}

impl Literal {
    pub fn new(value: String) -> Literal {
        Literal { value }
    }
}
