// Parsing rules
// expression -> equality;
// equality -> comparison ( ( "!=" | "==" ) comparison )* ;
// comparison ->  term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
// term ->  factor ( ( "-" | "+" ) factor )* ;
// factor -> unary ( ( "/" | "*" ) unary )* ;
// unary -> ( "!" | "-" ) unary | primary ;
// primary →> NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;

use std::fmt::Error;

use crate::{
    ast::expression::BinaryExp,
    lexemes::{token::Token, token_type::TokenType},
};

use crate::ast::expression::{Expression, GroupExp, LiteralExp, UnaryExp};

struct Parser {
    pub tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn new(&mut self, tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn expression(&mut self) -> Box<Expression> {
        self.equality()
    }

    fn equality(&mut self) -> Box<Expression> {
        let mut expr = self.comaprison();

        while self.match_tokens(vec![TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
            let operator = self.previous();
            let right = self.comaprison();

            let temp_expr = Expression::Binary(Box::new(BinaryExp::new(expr, operator, right)));

            expr = Box::new(temp_expr)
        }

        expr
    }

    fn comaprison(&mut self) -> Box<Expression> {
        let mut expr = self.term();

        while self.match_tokens(vec![
            TokenType::GREATER,
            TokenType::GREATER_EQUAL,
            TokenType::LESS,
            TokenType::LESS_EQUAL,
        ]) {
            let operator = self.previous();
            let right = self.term();

            let temp_expr = Expression::Binary(Box::new(BinaryExp::new(expr, operator, right)));

            expr = Box::new(temp_expr);
        }

        expr
    }

    fn term(&mut self) -> Box<Expression> {
        let mut expr = self.factor();

        while self.match_tokens(vec![TokenType::MINUS, TokenType::PLUS]) {
            let operator = self.previous();
            let right = self.factor();
            let temp_expr = Expression::Binary(Box::new(BinaryExp::new(expr, operator, right)));

            expr = Box::new(temp_expr);
        }

        expr
    }

    fn factor(&mut self) -> Box<Expression> {
        let mut expr = self.unary();

        while self.match_tokens(vec![TokenType::SLASH, TokenType::STAR]) {
            let operator = self.previous();
            let right = self.unary();
            let temp_expr = Expression::Binary(Box::new(BinaryExp::new(expr, operator, right)));

            expr = Box::new(temp_expr);
        }

        expr
    }

    fn unary(&mut self) -> Box<Expression> {
        if self.match_tokens(vec![TokenType::SLASH, TokenType::STAR]) {
            let operator = self.previous();
            let right = self.unary();
            let temp_exp = Expression::Unary(Box::new(UnaryExp::new(operator, right)));
            return Box::new(temp_exp);
        }

        self.primary()
    }

    fn primary(&mut self) -> Box<Expression> {
        if self.match_tokens(vec![TokenType::FALSE]) {
            return Box::new(Expression::Literal(Box::new(LiteralExp::new(
                "false".to_string(),
            ))));
        }

        if self.match_tokens(vec![TokenType::TRUE]) {
            return Box::new(Expression::Literal(Box::new(LiteralExp::new(
                "true".to_string(),
            ))));
        }

        if self.match_tokens(vec![TokenType::NIL]) {
            return Box::new(Expression::Literal(Box::new(LiteralExp::new(
                "null".to_string(),
            ))));
        }

        if self.match_tokens(vec![TokenType::STRING, TokenType::NUMBER]) {
            return Box::new(Expression::Literal(Box::new(LiteralExp::new(
                self.previous().lexeme,
            ))));
        }

        if self.match_tokens(vec![TokenType::LEFT_PAREN]) {
            let expr = self.expression();
            let _ = self.consume(TokenType::RIGHT_PAREN, "xpect ')' after expression.");

            return Box::new(Expression::Group(Box::new(GroupExp::new(expr))));
        }

        Box::new(Expression::Literal(Box::new(LiteralExp::new(
            "null".to_string(),
        ))))
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token, Error> {
        if self.check(token_type) {
            return Ok(self.advance());
        }

        panic!("Token {:?}; Message {}", self.peek(), message);
    }

    fn match_tokens(&mut self, types: Vec<TokenType>) -> bool {
        for token_type in types {
            self.check(token_type);
            self.advance();

            return true;
        }

        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().id == token_type
    }

    fn is_at_end(&self) -> bool {
        self.peek().id == TokenType::EOF
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            let temp = self.current + 1;
            self.current = temp;
        }

        self.previous()
    }

    fn previous(&self) -> Token {
        match self.tokens.get(self.current - 1) {
            Some(token) => token.clone(),
            None => Token::new(TokenType::EOF, String::new(), self.current, None),
        }
    }

    fn peek(&self) -> Token {
        match self.tokens.get(self.current) {
            Some(token) => token.clone(),
            None => Token::new(TokenType::EOF, String::new(), self.current, None),
        }
    }

    fn synchronise(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().id == TokenType::SEMICOLON {
                return;
            }

            match self.peek().id {
                TokenType::CLASS
                | TokenType::FUNC
                | TokenType::VAR
                | TokenType::FOR
                | TokenType::IF
                | TokenType::WHILE
                | TokenType::PRINT
                | TokenType::RETURN => return,
                _ => print!(""),
            }

            self.advance();
        }
    }
}
