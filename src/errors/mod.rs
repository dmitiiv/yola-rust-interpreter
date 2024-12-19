use core::fmt;

#[derive(Debug)]

pub struct YolaParseError {
    message: String,
}

impl YolaParseError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl fmt::Display for YolaParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parse Error: {}", self.message)
    }
}

impl std::error::Error for YolaParseError {}
