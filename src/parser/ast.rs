// basic operators
#[derive(Debug)]
pub enum Operator {
    Add,
    Subtraction,
    Multiply,
    Division, 
}

#[derive(Debug)]
pub enum Statement {
    Expression(Expr)
}

// basic AST to expressions representation
#[derive(Debug)]    
pub enum Expr {
    Number(i32), 
    Binary {
        left: Box<Expr>, 
        op: Operator, 
        right: Box<Expr>,
    },
}