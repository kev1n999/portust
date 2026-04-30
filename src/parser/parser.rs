use crate::lexer::tokens::{Token, TokenKind};
use crate::parser::ast::{Expr, Operator, Statement};

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0, }
    }
    // get the current token  
    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }
    // advance to the next token
    pub fn advance(&mut self) -> &Token {
        if self.current < self.tokens.len() {
            self.current += 1;
        }
        self.previous()
    }
    pub fn previous(&self) -> &Token {
        &self.tokens[self.current -1]
    }
    pub fn match_token(&mut self, expected_token_type: &[TokenKind]) -> Option<Operator> {
        if let Some(token) = self.peek() {
            for kind in expected_token_type {
                if *kind == token.token_type {
                    self.advance();
                    return match kind {
                        TokenKind::Plus => Some(Operator::Add),
                        TokenKind::Minus => Some(Operator::Subtraction),
                        TokenKind::Star => Some(Operator::Multiply),
                        TokenKind::Slash => Some(Operator::Division),
                        _ => None,
                    }
                }
            }
        }
        None
    }
    pub fn parse_number(&mut self) -> Result<Expr, String> {
        match self.peek() {
            Some(token) => {
                match token.token_type {
                    TokenKind::Number(v) => {
                        self.advance();
                        Ok(Expr::Number(v))
                    },
                    _ => Err(format!("Expected number, received {:#?}", token.token_type)),
                }
            },
            None => Err("An error ocurred!".to_string())
        }
    }
    // method to consume the semicolon delimiter(;)
    pub fn consume_delimiter(&mut self) {
        match self.peek() {
            Some(token) => {
                if token.token_type == TokenKind::SemiColon {
                    self.advance();
                }
            },
            None => { self.advance(); }
        }
    }
    pub fn parse_statement(&mut self) -> Option<Statement> {
        match self.parse_expression() {
            Ok(expr) => {
                self.consume_delimiter(); 
                return Some(Statement::Expression(expr))
            },
            Err(_) => None,
        }
    }
    pub fn parse_expression(&mut self) -> Result<Expr, String> {
        match self.parse_number() {
            Ok(expr) => {
                let mut left = expr;
                while let Some(operator) = self.match_token(&[
                    TokenKind::Plus, TokenKind::Minus, 
                    TokenKind::Star, TokenKind::Slash,
                ]) {
                    let right = self.parse_number()?;
                    left = Expr::Binary {
                        left: Box::new(left),
                        op: operator,
                        right: Box::new(right),
                    }
                }
                Ok(left)
            },
            Err(err) => Err(format!("An error ocurred!\n{:#?}", err)),
        }
    }
}