#[derive(Debug, PartialEq)]
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
    Print,
    Colon,
    Comma,
    SemiColon,
    LeftBrace,
    RightBrace,
    EOF,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenKind,
    pub lexeme: String,
}