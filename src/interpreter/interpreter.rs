use std::collections::HashMap;
use crate::parser::ast::{Expr, Operator};

pub struct Interpreter {
    pub env: HashMap<String, i32>, 
}

impl Interpreter {
    pub fn new() -> Self {
        Self { env: HashMap::new(),  }
    }
    pub fn eval(&mut self, expr: Expr) -> i32 {
        match expr {
            Expr::Number(num) => num, 
            Expr::Binary { left, op, right } => {
                let l = self.eval(*left);
                let r = self.eval(*right); 

                match op {
                    Operator::Add => l + r,
                    Operator::Subtraction => l - r, 
                    Operator::Multiply => l * r, 
                    Operator::Division => l / r, 
                } 
            },
            Expr::Assign { name, value } => {
               let val = self.eval(*value);
               self.env.insert(name, val);
               val
            }
        }
    }
}