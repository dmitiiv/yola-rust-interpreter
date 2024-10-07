use crate::lexemes::{literal::Literal, token::Token, token_type::TokenType};
use crate::report::report::{ErrorTypes, Report};
use crate::utils::string_utils::{CharAt, Slice};

// const BASE_REG_EXP: &str = "/[a-zA-Z_][a-zA-Z_0-9]*/";
pub struct Scanner {
    pub tokens: Vec<Token>,
    source: String,
    start: usize,
    current: usize,
    line: usize,
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
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
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
            '!' => {
                let token = if self.matcher('=') {
                    TokenType::BANG_EQUAL
                } else {
                    TokenType::BANG
                };
                self.add_token(token);
            }
            '=' => {
                let token = if self.matcher('=') {
                    TokenType::EQUAL_EQUAL
                } else {
                    TokenType::EQUAL
                };
                self.add_token(token);
            }
            '<' => {
                let token = if self.matcher('=') {
                    TokenType::LESS_EQUAL
                } else {
                    TokenType::LESS
                };
                self.add_token(token);
            }
            '>' => {
                let token = if self.matcher('=') {
                    TokenType::GREATER_EQUAL
                } else {
                    TokenType::GREATER
                };
                self.add_token(token);
            }
            '/' => {
                if self.matcher('/') {
                    while self.peek() != '\n' && self.is_at_end() {
                        self.advance();
                    }
                }

                self.add_token(TokenType::SLASH)
            }
            ' ' | '\r' | '\t' => (),
            '\n' => {
                let temp = self.line + 1;
                self.line = temp;
            }
            '"' => self.string(),
            _ => {
                let err_mes = format!("Unexpected character: {}", char);

                Report::error(None, ErrorTypes::TypeErr.as_str(), self.line, err_mes)
            }
        };
    }

    fn advance(&mut self) -> char {
        let temp = self.current + 1;
        self.current = temp;

        self.source.char_at(self.current - 1)
    }

    fn add_token(&mut self, id: TokenType) {
        self.create_token(id, None)
    }

    fn create_token(&mut self, id: TokenType, literal: Option<Literal>) {
        let text = self.source.slice(self.start, self.current);

        let token = Token::new(id, text, self.line, literal);

        self.tokens.push(token);
    }

    fn matcher(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        } else if self.source.char_at(self.current) != expected {
            return false;
        } else {
            let temp = self.current + 1;
            self.current = temp;
            true
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        } else {
            self.source.char_at(self.current)
        }
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                let temp = self.line + 1;
                self.line = temp;
            }

            if self.is_at_end() {
                let err_mes = String::from("Cannot find close sign for string");
                Report::error(None, ErrorTypes::SynErr.as_str(), self.line, err_mes)
            }

            self.advance();

            let value = self.source.slice(self.start + 1, self.current - 1);

            self.create_token(TokenType::STRING, Some(Literal::new(value)));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::scanner;

    use super::*;

    #[test]
    fn test_left_parem() {
        let source = String::from("(");

        let mut scanner = Scanner::new(source.clone());

        let tokens = scanner.scan_tokens();

        assert_eq!(tokens.len(), 2);

        let token = &tokens[0];

        assert_eq!(token.id, TokenType::LEFT_PAREN);
        assert_eq!(&token.lexeme, &source);
    }
}
