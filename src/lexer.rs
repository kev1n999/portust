use crate::token_kind::{TokenKind, Token};

pub struct Lexer {
    pub input: Vec<char>,
    pub current: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self { 
        Lexer { input: source.chars().collect(), current: 1,} 
    }
}