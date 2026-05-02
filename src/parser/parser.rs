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
    pub fn expected_token(&mut self, expected_token: TokenKind) -> Option<TokenKind> {
        if let Some(token) = self.peek() {
            if token.token_type == expected_token {
                let cloned_token = token.token_type.clone(); 
                self.advance(); 
                return Some(cloned_token);
            } 
        }
        None
    } 
    pub fn parse_operator(&mut self, expected_token_type: &[TokenKind]) -> Option<Operator> {
        for kind in expected_token_type {
            if let Some(token_type) = self.expected_token(kind.clone()) {
                return match token_type {
                    TokenKind::Plus => Some(Operator::Add),
                    TokenKind::Minus => Some(Operator::Subtraction),
                    TokenKind::Star => Some(Operator::Multiply),
                    TokenKind::Slash => Some(Operator::Division),
                    _ => None,
                };
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
    pub fn consume<F>(&mut self, check: F, err_msg: &str) -> Result<Token, String> 
        where F: Fn(&TokenKind) -> bool
    {
        if let Some(token) = self.peek() {
            if check(&token.token_type) {
                let cloned_token = token.clone(); 
                self.advance();
                return Ok(cloned_token);
            }
        }
        Err(err_msg.to_string())
    }
    pub fn parse_statement(&mut self) -> Option<Statement> {
        match self.parse_expression() {
            Ok(expr) => {
                if let Err(err) = self.consume(|t| matches!(t, TokenKind::SemiColon), "Expected SemiColon ';'") {
                    panic!("{:?}", &err);
                }
                return Some(Statement::Expression(expr))
            },
            Err(_) => None,
        }
    }
    pub fn parse_expression(&mut self) -> Result<Expr, String> {
        match self.parse_number() {
            Ok(expr) => {
                let mut left = expr;
                while let Some(operator) = self.parse_operator(&[
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