use crate::lexer::tokens::{Token, TokenKind};

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
    pub fn parse_number(&mut self) -> Token {
        let mut founded_value = String::new(); 
        while let Some(current) = self.peek() {
            if current.is_ascii_digit() {
                founded_value.push(current);
                self.advance();
            } else { break; }
        }
        Token { token_type: TokenKind::Number(founded_value.parse::<i32>().unwrap()), lexeme: founded_value, }
    }
    pub fn parse_identifier(&mut self) -> Token {
        let mut identf = String::new();
        while let Some(current) = self.peek() {
            if current.is_alphanumeric() || current == '_' {
                identf.push(current);
                self.advance();
            } else { break; }
        }
        let token_type = match identf.as_str() {
            "escreva" => TokenKind::Print, 
            "funcao" => TokenKind::Function,
            "verdaeiro" => TokenKind::True,
            "falso" => TokenKind::False,
            _ => TokenKind::Identifier(identf.clone()),
        };
        Token { token_type: token_type, lexeme: identf, }
    }
    pub fn parse_string(&mut self) -> Token {
        self.advance();
        let mut str_val = String::new();
        while let Some(current) = self.peek() {
            if current == '"' {
                break; 
            }
            str_val.push(current);
            self.advance();
        }
        self.advance();
        Token { token_type: TokenKind::String(str_val.clone()), lexeme: str_val.clone(), }
    }
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        match self.peek() {
            Some(c) if c.is_ascii_digit() => self.parse_number(),
            Some(c) if c.is_alphabetic() => self.parse_identifier(),
            Some('"') => self.parse_string(),
            Some('(') => {
                self.advance();
                Token { token_type: TokenKind::LParen, lexeme: "(".to_string(), }
            },
            Some(')') => {
                self.advance();
                Token { token_type: TokenKind::RParen, lexeme: ")".to_string(), }
            },
            Some(':') => {
                self.advance();
                Token { token_type: TokenKind::Colon, lexeme: ":".to_string(), }
            },
            Some(',') => {
                self.advance();
                Token { token_type: TokenKind::Comma, lexeme: ",".to_string(), }
            },
            Some('{') => {
                self.advance();
                Token { token_type: TokenKind::LeftBrace, lexeme: "{".to_string(), }
            },
            Some('}') => {
                self.advance();
                Token { token_type: TokenKind::RightBrace, lexeme: "}".to_string(), }
            },
            Some('=') => {
                self.advance();
                Token { token_type: TokenKind::Equals, lexeme: "=".to_string(), }
            },
            Some('+') => {
                self.advance();
                Token { token_type: TokenKind::Plus, lexeme: "+".to_string(), }
            },
            Some(';') => {
                self.advance();
                Token { token_type: TokenKind::SemiColon, lexeme: ";".to_string(), }
            },
            _ => Token { token_type: TokenKind::EOF, lexeme: "".to_string(), }
        }
    }
}