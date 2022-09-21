
#[derive(Debug, PartialEq)]
pub enum ExprNode{
    Integer(i64),
    Float(f64),
    Op(Box<ExprNode>, Op, Box<ExprNode>),
    UnaryOp(UnaryOp, Box<ExprNode>)
}

#[derive(Debug, PartialEq)]
pub enum Op{
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Lt,
    Le,
    Eq,
    Ne,
    Gt,
    Ge,
    InfixFn(String)
}

#[derive(Debug, PartialEq)]
pub enum UnaryOp{
    Plus,
    Minus,
    Not
}

#[derive(Debug)]
pub enum StmtNode{
    ExprStmt(Box<ExprNode>),
    ReturnStmt
}

#[derive(Debug)]
pub struct FunctionDefinition {
    pub name: String,
    pub private: bool,
    pub args: Vec<(String, String)>,
    pub stmts: Vec<StmtNode>,
}
