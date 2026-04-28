mod lexer;
mod parser;

use crate::lexer::lexer::{Lexer};
use crate::lexer::tokens::{TokenKind};

fn main() {
    let source = r#"
    funcao blablbalaalal(x, y) {
        true_test = verdadeiro;
        false_test = falso; 
        str_test = "hello world";
    }"#;
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