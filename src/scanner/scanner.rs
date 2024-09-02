use crate::lexemes::{literal::Literal, token::Token, token_type::TokenType};

const BASE_REG_EXP: &str = "/[a-zA-Z_][a-zA-Z_0-9]*/";
pub struct Scanner {
    tokens: Vec<Token>,
    source: String,
    start: usize,
    current: usize,
    line: i32,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while self.is_at_end() {
            self.start = self.current;
            self.scan_token()
        }

        let token = Token::new(TokenType::EOF, String::from(""), self.line, None);
        self.tokens.push(token);

        &self.tokens
    }

    fn scan_token(&mut self) {
        let char = self.advance();

        match char {
            '(' => self.add_token(TokenType::LEFT_PAREN),
            ')' => self.add_token(TokenType::RIGHT_BRACE),
            '{' => self.add_token(TokenType::LEFT_BRACE),
            '}' => self.add_token(TokenType::RIGHT_BRACE),
            ',' => self.add_token(TokenType::COMMA),
            '.' => self.add_token(TokenType::DOT),
            '+' => self.add_token(TokenType::PLUS),
            '-' => self.add_token(TokenType::MINUS),
            ';' => self.add_token(TokenType::SEMICOLON),
            '*' => self.add_token(TokenType::STAR),
            _ => self.add_token(TokenType::AND),
        };
    }

    fn advance(&mut self) -> char {
        self.current = self.current + 1;

        match self.source.chars().nth(self.current - 1) {
            Some(character) => character,
            None => '_',
        }
    }

    fn add_token(&mut self, id: TokenType) {
        self.create_token(id, None)
    }

    fn create_token(&mut self, id: TokenType, literal: Option<Literal>) {
        let chars: Vec<char> = self.source.chars().collect();

        let text: String = chars[self.start..self.current].iter().collect();

        let token = Token::new(id, text, self.line, literal);

        self.tokens.push(token);
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
