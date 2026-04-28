mod lexer;
mod parser;

use crate::lexer::lexer::Lexer;
use crate::lexer::tokens::TokenKind;
use crate::parser::parser::Parser;

fn main() {
    let source = "10 + 10";
    
    let mut all_tokens = Vec::new();
    let mut lex = Lexer::new(source);

    loop {
        let token = lex.next_token();
        if token.token_type == TokenKind::EOF { break; }
        all_tokens.push(token);
    }

    let mut parser = Parser::new(all_tokens);
    let expr = parser.parse_expression().unwrap();
    println!("{:#?}", expr);
}   