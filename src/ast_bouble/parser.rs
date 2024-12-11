// Parsing rules
// expression -> equality;
// equality -> comparison ( ( "!=" | "==" ) comparison )* ;
// comparison ->  term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
// term ->  factor ( ( "-" | "+" ) factor )* ;
// factor -> unary ( ( "/" | "*" ) unary )* ;
// unary -> ( "!" | "-" ) unary | primary ;
// primary →> NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;

use crate::{
    ast_bouble::{expression::BinaryExp, expression::Expr},
    lexemes::{
        token::Token,
        token_type::{self, TokenType},
    },
};

use super::expression::Expression;

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

            let temp_expr = BinaryExp::new(expr, operator, right);

            expr = Box::new(temp_expr);
        }

        expr
    }

    fn comaprison<T>(&mut self) -> Box<T> {}

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
}
