use crate::lexer::tokens::{Token};

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
    pub fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }
    // advance to the next token
    pub fn advance(&mut self) -> () {
        self.current += 1
    }
}