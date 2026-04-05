mod lexer;
mod token_kind; 

use crate::{lexer::Lexer, token_kind::TokenKind};

fn main() {
    let source = "10";
    let mut lex = Lexer::new(source);
    
    loop {
        let token = lex.next_token();
        if token.token_type == TokenKind::EOF {
            println!("{:#?}", token);
            break;
        }
        println!("{:#?}", token);
    }
}   