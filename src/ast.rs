#[derive(Debug)]
pub enum Expr {
    Int(i64),
    Float(f64),
    Bool(bool),
    Identifer(String),
    Str(String),
    OpExpr(Box<Expr>, Opcode, Box<Expr>),
    UnaryOpExpr(Opcode, Box<Expr>)
}


#[derive(Debug)]
pub enum Opcode {
    Not,
    Neg,
    BitNot,

    Lsh,
    Rsh,
    RshUnsigned,
    BitAnd,
    BitOr,
    BitXor,

    Mul,
    Div,
    Mod,
    Pow,
    MulAssign,
    DivAssign,
    ModAssign,

    Add,
    Sub,
    AddAssign,
    SubAssign,


    Eq,
    Lt,
    Gt,
    Le,
    Ge,

    And,
    Or,
}
