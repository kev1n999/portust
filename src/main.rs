mod lexer;
mod parser;
mod interpreter;

use crate::interpreter::interpreter::Interpreter;
use crate::lexer::lexer::Lexer;
use crate::lexer::tokens::TokenKind;
use crate::parser::ast::Statement;
use crate::parser::parser::Parser;

fn main() {
    let source = r#"
        x = 10 - 10;
        y = 23 + 43;
        z = 324 / 34;
        w = 34 * 342;
    "#;

    let mut tokens = Vec::new();
    let mut lexer = Lexer::new(source);

    loop {
        let token = lexer.next_token();
        if token.token_type == TokenKind::EOF { break; }
        tokens.push(token);
    }

    let mut parser = Parser::new(tokens);
    let mut statements = Vec::new();

    loop {
        match parser.parse_statement() {
            Some(stt) => {
                statements.push(stt);
            },
            _ => break,
        }
    }

    // println!("{:#?}", statements);
    let mut interpreter = Interpreter::new();
    
    for stt in statements {
        match stt {
            Statement::Expression(expr) => {
                println!("{}", interpreter.eval(expr));
            },
        }
    }
    
}