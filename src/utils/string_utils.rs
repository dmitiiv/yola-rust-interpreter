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

pub trait Slice {
    fn slice(&self, start: usize, end: usize) -> String;
}

impl Slice for String {
    fn slice(&self, start: usize, end: usize) -> String {
        // add end and start validation

        let chars: Vec<char> = self.chars().collect();
        chars[start..end].iter().collect()
    }
}
