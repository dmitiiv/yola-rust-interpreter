pub trait CharAt {
    fn char_at(&self, index: usize) -> char;
}

impl CharAt for &str {
    fn char_at(&self, index: usize) -> char {
        self.chars().nth(index).unwrap()
    }
}

impl CharAt for String {
    fn char_at(&self, index: usize) -> char {
        self.chars().nth(index).unwrap()
    }
}
