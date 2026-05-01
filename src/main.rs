mod lexer;
mod parser;
mod interpreter;

use crate::interpreter::interpreter::Interpreter;
use crate::lexer::lexer::Lexer;
use crate::lexer::tokens::TokenKind;
use crate::parser::ast::{Expr, Statement};
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
    let mut statements: Vec<Statement> = Vec::new();

    loop {
        let token = lex.next_token();
        if token.token_type == TokenKind::EOF { break; }
        all_tokens.push(token);
    }

    let mut parser = Parser::new(all_tokens);

    loop {
        let sttm = parser.parse_statement();
        if let Some(statement) = sttm {
            statements.push(statement);
        } else { break; }
    }

    let mut interpreter = Interpreter::new();
    for sttm in statements {
        match sttm {
            Statement::Expression(expr) => {
                let result: i32 = interpreter.eval(expr);
                println!("{}", result);
            }
        }
    }
}   