use crate::lexer::tokens::{Token, TokenKind};
use crate::parser::ast::{Expr, Operator};

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
    pub fn match_token(&mut self, expected_token_type: TokenKind) -> Option<Operator> {
        match self.peek() {
            Some(token) => {
                if token.token_type == expected_token_type {
                    self.advance();

                    return match expected_token_type {
                        TokenKind::Plus => Some(Operator::Add),
                        _ => None,
                    }
                }
            },
            _ => eprintln!("")
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
    pub fn parse_expression(&mut self) -> Result<Expr, String> {
        match self.parse_number() {
            Ok(expr) => {
                let mut left = expr;
                while let Some(operator) = self.match_token(TokenKind::Plus) {
                    if let Ok(right) = self.parse_number() {
                        left = Expr::Binary {
                            left: Box::new(left),
                            op: operator,
                            right: Box::new(right),
                        }
                    }
                }
                Ok(left)
            },
            Err(err) => Err(format!("An error ocurred!\n{:#?}", err)),
        }
    }
}