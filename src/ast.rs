

#[derive(Debug)]
pub enum Expr {
    // Integer literal.
    Int(isize),

    // Unary minus.
    Neg(Box<Expr>),

    // Binary operators.
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}