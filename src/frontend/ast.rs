use crate::frontend::func::{ProgramClassElement, ProgramSrcFnElement};

pub type Block = Vec<StmtNode>;

#[derive(Debug, PartialEq, Clone)]
pub enum StmtNode {
    ExprStmt(Box<ExprNode>),
    RetStmt(Option<Box<ExprNode>>),
    VarStmt(String, Option<TypeInfo>, bool, Box<ExprNode>),
    WhileStmt(Box<ExprNode>, Block)
}

pub type AccessedIdent = Vec<String>;

#[derive(Debug, PartialEq, Clone)]
pub enum ExprNode {
    Integer(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Ident(AccessedIdent),
    Op(Box<ExprNode>, Op, Box<ExprNode>),
    FnCall(AccessedIdent, Option<Vec<Box<ExprNode>>>),
    UnaryOp(UnaryOp, Box<ExprNode>),
    BlockExpr(Block),
    AssignExpr(String, Box<ExprNode>),
    //last stmt is return value
    IfExpr(Box<ExprNode>, Box<ExprNode>, Option<Box<ExprNode>>),
}

#[derive(Debug, PartialEq, Clone)]
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
    And,
    Or,
    InfixFn(String),
}

#[derive(Debug, PartialEq, Clone)]
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
    Any,
    TypeSym(String),
}


impl ToString for TypeInfo {
    fn to_string(&self) -> String {
        match self {
            TypeInfo::Int => String::from("int"),
            TypeInfo::Float => String::from("float"),
            TypeInfo::Bool => String::from("bool"),
            TypeInfo::Unit => String::from("unit"),
            TypeInfo::Any => String::from("any"),
            TypeInfo::TypeSym(sym) => format!(".{}", sym)
        }
    }
}


impl TypeInfo {
    pub fn is_can_convert_to(&self, target: &TypeInfo) -> bool {
        self == target
            || (self == &TypeInfo::Int && target == & TypeInfo::Float)
            || target == &TypeInfo::Any
    }
}

impl From<String> for TypeInfo {
    fn from(tok: String) -> Self {
        match tok.as_str() {
            "int" => TypeInfo::Int,
            "float" => TypeInfo::Float,
            "bool" => TypeInfo::Bool,
            "unit" => TypeInfo::Unit,
            _ => TypeInfo::TypeSym(tok)
        }
    }
}

impl From<&str> for TypeInfo {
    fn from(tok: &str) -> Self {
        match tok {
            "int" => TypeInfo::Int,
            "float" => TypeInfo::Float,
            "bool" => TypeInfo::Bool,
            "unit" => TypeInfo::Unit,
            oth => TypeInfo::TypeSym(String::from(oth))
        }
    }
}

#[derive(Debug)]
pub struct FunctionDefinition {
    pub name: String,
    pub private: bool,
    pub args: Vec<(String, String)>,
    pub stmts: Vec<StmtNode>,
}

#[derive(Debug, PartialEq)]
pub enum ProgramRootElement {
    Import(String),
    Function(ProgramSrcFnElement),
    Class(ProgramClassElement)
}

impl ProgramRootElement {
    pub fn modify_to_module(self, module_name: String) -> Self {
        match self {
            ProgramRootElement::Import(_) => self,
            ProgramRootElement::Function(mut e) => {
                e.header.module = Some(module_name);
                ProgramRootElement::Function(e)
            }

            ProgramRootElement::Class(mut e) => {
                e.module = module_name;
                ProgramRootElement::Class(e)
            }
        }
    }
}

