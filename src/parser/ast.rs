// basic operators
#[derive(Debug)]
pub enum Operator {
    Add,
    Subtraction,
    Multiply,
    Division, 
}

// basic AST to expressions representation
#[derive(Debug)]    
pub enum Expr {
    Number(f64), 
    Binary {
        left: Box<Expr>, 
        op: Operator, 
        right: Box<Expr>,
    },
}