#[derive(Debug)]
pub enum Expr {
    Int(i64),
    Float(f64),
    Bool(bool),
    Identifer(String),
    Str(String),
    OpExpr(Box<Expr>, Opcode, Box<Expr>),
    UnaryOpExpr(Opcode, Box<Expr>),
    BlockExpr(Block),
    If{
        main: (Box<Expr>, Box<Expr>),
        elif:Vec<(Box<Expr>, Box<Expr>)>,
        els: Option<Box<Expr>>,
    },
}

#[derive(Debug)]
pub enum Stmt {
    ExprStmt(Box<Expr>),
    AssignStmt { name: String, value: Box<Expr> },
    DeclStmt(Decl),
    RepeatUntilStmt(Box<Expr>, Block),
    WhileStmt(Box<Expr>, Block),
    ForEachStmt(String, Box<Expr>, Block),
    ReturnStmt(Option<Box<Expr>>),
}

#[derive(Debug)]
pub enum Decl {
    Val {
        name: String,
        value: Box<Expr>,
        decl_type: String,
    },
    Var {
        name: String,
        value: Box<Expr>,
        decl_type: String,
    },
}

#[derive(Debug)]
pub struct Block {
    pub stmts: Vec<Stmt>,
}

#[derive(Debug)]
pub enum ProgramElem {
    VarDef(Decl),
    Import(String),
    FuncDef(FunctionDef),
}

#[derive(Debug)]
pub struct FunctionDef {
    pub return_type: String,
    pub name: String,
    pub block: Block,
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
