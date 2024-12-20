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
    errors::YolaParseError,
    lang::yola::Yola,
    lexemes::{token::Token, token_type::TokenType},
};

use crate::ast::expression::{Expression, GroupExp, LiteralExp, UnaryExp};

pub struct Parser {
    pub tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Option<Box<Expression>> {
        match self.expression() {
            Ok(expr) => Some(expr),
            Err(err) => {
                println!("{}", err);
                None
            }
        }
    }

    fn expression(&mut self) -> Result<Box<Expression>, YolaParseError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Box<Expression>, YolaParseError> {
        let mut expr = self.comaprison()?;

        while self.match_tokens(vec![TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
            let operator = self.previous();
            let right = self.comaprison()?;

            let temp_expr = Expression::Binary(Box::new(BinaryExp::new(expr, operator, right)));

            expr = Box::new(temp_expr)
        }

        Ok(expr)
    }

    fn comaprison(&mut self) -> Result<Box<Expression>, YolaParseError> {
        let mut expr = self.term()?;

        while self.match_tokens(vec![
            TokenType::GREATER,
            TokenType::GREATER_EQUAL,
            TokenType::LESS,
            TokenType::LESS_EQUAL,
        ]) {
            let operator = self.previous();
            let right = self.term()?;

            let temp_expr = Expression::Binary(Box::new(BinaryExp::new(expr, operator, right)));

            expr = Box::new(temp_expr);
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Box<Expression>, YolaParseError> {
        let mut expr = self.factor()?;

        while self.match_tokens(vec![TokenType::MINUS, TokenType::PLUS]) {
            let operator = self.previous();
            let right = self.factor()?;
            let temp_expr = Expression::Binary(Box::new(BinaryExp::new(expr, operator, right)));

            expr = Box::new(temp_expr);
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Box<Expression>, YolaParseError> {
        let mut expr = self.unary()?;

        while self.match_tokens(vec![TokenType::SLASH, TokenType::STAR]) {
            let operator = self.previous();
            let right = self.unary()?;
            let temp_expr = Expression::Binary(Box::new(BinaryExp::new(expr, operator, right)));

            expr = Box::new(temp_expr);
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Box<Expression>, YolaParseError> {
        if self.match_tokens(vec![TokenType::SLASH, TokenType::STAR]) {
            let operator = self.previous();
            let right = self.unary()?;
            let temp_exp = Expression::Unary(Box::new(UnaryExp::new(operator, right)));
            return Ok(Box::new(temp_exp));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Box<Expression>, YolaParseError> {
        if self.match_tokens(vec![TokenType::FALSE]) {
            let res_expr = Box::new(Expression::Literal(Box::new(LiteralExp::new(
                "false".to_string(),
            ))));

            return Ok(res_expr);
        }

        if self.match_tokens(vec![TokenType::TRUE]) {
            let res_expr = Box::new(Expression::Literal(Box::new(LiteralExp::new(
                "true".to_string(),
            ))));

            return Ok(res_expr);
        }

        if self.match_tokens(vec![TokenType::NIL]) {
            let res_expr = Box::new(Expression::Literal(Box::new(LiteralExp::new(
                "null".to_string(),
            ))));

            return Ok(res_expr);
        }

        if self.match_tokens(vec![TokenType::STRING, TokenType::NUMBER]) {
            let res_expr = Box::new(Expression::Literal(Box::new(LiteralExp::new(
                self.previous().lexeme,
            ))));

            return Ok(res_expr);
        }

        if self.match_tokens(vec![TokenType::LEFT_PAREN]) {
            let expr = self.expression()?;
            let _ = self.consume(TokenType::RIGHT_PAREN, "xpect ')' after expression.");

            let res_expr = Box::new(Expression::Group(Box::new(GroupExp::new(expr))));
            return Ok(res_expr);
        }

        return Err(YolaParseError::new(format!(
            "Expect expression {}",
            self.peek().lexeme
        )));
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
