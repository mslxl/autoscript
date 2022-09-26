use std::collections::HashMap;
use crate::frontend::ast::TypeInfo::TypeSym;

pub type Block = Vec<StmtNode>;

#[derive(Debug, PartialEq, Clone)]
pub enum StmtNode {
    ExprStmt(Box<ExprNode>),
    RetStmt(Option<Box<ExprNode>>),
    VarStmt(String, Option<TypeRef>, bool, Box<ExprNode>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionHeader{
    pub name:String,
    pub modules: Option<String>,
    pub param: Option<Vec<(String,TypeRef)>>,
    pub ret:Option<TypeRef>
}

impl FunctionHeader{
    pub fn signature(&self) -> String{
        let ret = self.ret.as_ref().map(|x| x.0.clone()).unwrap_or(String::from("V"));
        let name = self.name.clone();
        let param = match self.param {
            Some(ref params) => {
                if params.len() > 1 {
                    params.iter()
                        .map(|x| x.1.0.clone())
                        .reduce(|a,b| format!("{},{}",a,b))
                        .unwrap()
                }else if !params.is_empty(){
                    params.first().unwrap().1.0.clone()
                }else{
                    String::from("V")
                }
            },
            None => String::from("V")
        };
        format!("{}@{}({}", ret, name, param)
    }
}

impl ToString for FunctionHeader{
    fn to_string(&self) -> String {
        self.signature()
    }
}

#[derive(Clone)]
pub struct ProgramSrcModule {
    pub function: HashMap<String, Vec<ProgramSrcFnElement>>
}

#[derive(Debug, PartialEq, Clone)]
pub struct ProgramSrcFnElement {
    pub header: FunctionHeader,
    pub block: Block,
}

#[derive(Debug, PartialEq)]
pub enum ProgramSrcElement {
    Import(String),
    Function(ProgramSrcFnElement)
}

#[derive(Debug, PartialEq, Clone)]
pub struct TypeRef(pub String);

#[derive(Debug, PartialEq, Clone)]
pub enum ExprNode {
    Integer(i64),
    Float(f64),
    Ident(String),
    Op(Box<ExprNode>, Op, Box<ExprNode>),
    UnaryOp(UnaryOp, Box<ExprNode>),
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
    TypeSym(String),
}

impl TypeInfo{
    pub fn is_can_convert_to(&self, target: &TypeInfo) -> bool{
        if self == target {
            true
        }else if self == &TypeInfo::Int && target == &TypeInfo::Float {
            true
        }else{
            false
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

