mod lexer;
mod parser;

use crate::lexer::lexer::Lexer;
use crate::lexer::tokens::TokenKind;
use crate::parser::parser::Parser;

fn main() {
    let source = r#"
        10 - 10; 
        23 + 43;  
        324 / 34; 
        34 * 342;
    "#;
    
    let mut all_tokens = Vec::new();
    let mut lex = Lexer::new(source);

    loop {
        let token = lex.next_token();
        if token.token_type == TokenKind::EOF { break; }
        all_tokens.push(token);
    }

    let mut parser = Parser::new(all_tokens);

    loop {
        let sttm = parser.parse_statement();
        if let Some(statement) = sttm {
            println!("{:?}", statement);
        } else { break; }
    }
}   