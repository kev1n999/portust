#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    True,
    False,
    Function,
    Identifier(String),
    Number(i32),
    String(String),
    LParen,
    RParen,
    Equals,
    Plus,
    Star,
    Slash,
    Minus,
    Print,
    Colon,
    Comma,
    SemiColon,
    LeftBrace,
    RightBrace,
    EOF,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenKind,
    pub lexeme: String,
}