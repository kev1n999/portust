#[derive(Debug, PartialEq)]
pub enum TokenKind {
    True,
    False,
    Function,
    Identifier,
    Number(i32),
    String(String),
    LParen,
    RParen,
    Equals,
    Plus,
    EOF,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenKind,
    pub lexeme: String,
}