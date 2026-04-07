mod lexer;
mod token_kind; 

use crate::{lexer::Lexer, token_kind::TokenKind};

fn main() {
    let source = r#"
    funcao blablbalaalal(x, y) {
        true_test = verdadeiro 
        false_test = falso
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