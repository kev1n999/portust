use crate::token_kind::{TokenKind, Token};

#[derive(Debug, PartialEq)]
pub struct Lexer {
    pub input: Vec<char>,
    pub current: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self { 
        Lexer { input: source.chars().collect(), current: 0,} 
    }
    pub fn peek(&self) -> Option<char> {
        self.input.get(self.current).copied()
    }
    pub fn advance(&mut self) -> () {
        self.current += 1;
    }
    pub fn skip_whitespace(&mut self) -> () {
        while let Some(current) = self.peek() {
            if current.is_whitespace() {
                self.advance();
            } else { break; }
        } 
    }
    pub fn number_parse(&mut self) -> Token {
        let mut founded_value = String::new(); 
        while let Some(current) = self.peek() {
            if current.is_ascii_digit() {
                founded_value.push(current);
                self.advance();
            } else { break; }
        }
        Token { token_type: TokenKind::Number(founded_value.parse::<i32>().unwrap()), lexeme: founded_value, }
    }
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        match self.peek() {
            Some(c) if c.is_ascii_digit() => self.number_parse(),
            _ => Token { token_type: TokenKind::EOF, lexeme: "".to_string(), }
        }
    }
}