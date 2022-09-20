
#[derive(Debug)]
pub enum ExprNode{
    Integer(i32),
    Float(f64),
    Op(Box<ExprNode>, String, Box<ExprNode>),
    UnaryOp(String, Box<ExprNode>)
}

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
