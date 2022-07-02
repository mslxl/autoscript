#[derive(Debug)]
pub enum Expr {
    Int(i64),
    Float(f64),
    OpExpr(Box<Expr>, Opcode, Box<Expr>),
}


#[derive(Debug)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,

    Mod,
    Pow,
}
