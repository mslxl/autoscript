use crate::frontend::ast::TypeInfo::TypeSym;

pub type Block = Vec<StmtNode>;

#[derive(Debug, PartialEq)]
pub enum StmtNode {
    ExprStmt(Box<ExprNode>),
    RetStmt(Option<Box<ExprNode>>),
    VarStmt(String, Option<TypeRef>, bool, Box<ExprNode>),
}

#[derive(Debug, PartialEq)]
pub enum Program {
    Function {
        name: String,
        param: Option<Vec<(String, TypeRef)>>,
        ret: Option<TypeRef>,
        block: Block,
    }
}

#[derive(Debug, PartialEq)]
pub struct TypeRef(pub String);

#[derive(Debug, PartialEq)]
pub enum ExprNode {
    Integer(i64),
    Float(f64),
    Op(Box<ExprNode>, Op, Box<ExprNode>),
    UnaryOp(UnaryOp, Box<ExprNode>),
}

#[derive(Debug, PartialEq)]
pub enum Op {
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
    InfixFn(String),
}

#[derive(Debug, PartialEq)]
pub enum UnaryOp {
    Plus,
    Minus,
    Not,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum TypeInfo {
    Int,
    Float,
    Bool,
    Unit,
    TypeSym(String),
}

impl From<&str> for TypeInfo {
    fn from(tok: &str) -> Self {
        match tok {
            "int" => TypeInfo::Int,
            "float" => TypeInfo::Float,
            "bool" => TypeInfo::Bool,
            "unit" => TypeInfo::Unit,
            oth => TypeSym(String::from(oth))
        }
    }
}

impl From<TypeRef> for TypeInfo{
    fn from(refer: TypeRef) -> Self {
        Self::from(refer.0.as_str())
    }
}

#[derive(Debug)]
pub struct FunctionDefinition {
    pub name: String,
    pub private: bool,
    pub args: Vec<(String, String)>,
    pub stmts: Vec<StmtNode>,
}

