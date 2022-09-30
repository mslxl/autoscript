use std::collections::HashMap;
use crate::vm::builtin::builtin_func::AutoScriptVmFnCode;

pub type Block = Vec<StmtNode>;

#[derive(Debug, PartialEq, Clone)]
pub enum StmtNode {
    ExprStmt(Box<ExprNode>),
    RetStmt(Option<Box<ExprNode>>),
    VarStmt(String, Option<TypeInfo>, bool, Box<ExprNode>),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FunctionOrigin {
    Source,
    VM,
    FFI,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionHeader {
    pub name: String,
    pub module: Option<String>,
    pub param: Option<Vec<(String, TypeInfo)>>,
    pub ret: Option<TypeInfo>,
    pub origin: FunctionOrigin,
}

pub trait FunctionCompare {
    fn is_executable_by(&self, name: &str, param: Option<&Vec<TypeInfo>>) -> bool;
}

impl FunctionCompare for FunctionHeader {
    fn is_executable_by(&self, name: &str, param: Option<&Vec<TypeInfo>>) -> bool {
        if self.name != name {
            false // Name is not matched!!!
        } else if let Some(ref self_param) = self.param {
            if let Some(param) = param {
                if self_param.len() != param.len() {
                    // Arguments number is not matched
                    false
                } else {
                    for i in 0..self_param.len() {
                        if !param[i].is_can_convert_to(&self_param[i].1) {
                            // A param can't be converted as requirement
                            return false;
                        }
                    }
                    true // All requirement is satisfied
                }
            } else {
                // Require arguments, but got no arguments
                false
            }
        } else {
            param == None
        }
    }
}

impl FunctionHeader {
    pub fn signature(&self) -> String {
        let ret = self.ret.as_ref().map(|x| x.to_string()).unwrap_or(String::from("V"));

        let name = self.name.clone();
        let module_name = self.module.as_deref().unwrap_or("");
        let param: String = match self.param {
            Some(ref params) => {
                if params.len() > 1 {
                    params.iter()
                        .map(|x| x.1.to_string())
                        .reduce(|a, b| format!("{},{}", a, b))
                        .unwrap()
                } else if !params.is_empty() {
                    params.first().unwrap().1.to_string()
                } else {
                    String::from("V")
                }
            }
            None => String::from("V")
        };

        let origin_flag = match self.origin {
            FunctionOrigin::Source => "",
            FunctionOrigin::VM => "~",
            FunctionOrigin::FFI => "#"
        };

        format!("{}{}@{}.{}({}", origin_flag, ret, module_name, name, param)
    }
}

impl ToString for FunctionHeader {
    fn to_string(&self) -> String {
        self.signature()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ProgramSrcFnElement {
    pub header: FunctionHeader,
    pub block: Block,
}

impl FunctionCompare for ProgramSrcFnElement {
    fn is_executable_by(&self, name: &str, param: Option<&Vec<TypeInfo>>) -> bool {
        self.header.is_executable_by(name, param)
    }
}

#[derive(Debug, Clone)]
pub struct ProgramVmFnElement {
    pub header: FunctionHeader,
    pub block: Box<dyn AutoScriptVmFnCode>,
}

impl FunctionCompare for ProgramVmFnElement{
    fn is_executable_by(&self, name: &str, param: Option<&Vec<TypeInfo>>) -> bool {
        self.header.is_executable_by(name, param)
    }
}

#[derive(Debug, PartialEq)]
pub enum ProgramSrcElement {
    Import(String),
    Function(ProgramSrcFnElement),
}

impl ProgramSrcElement {
    pub fn modify_to_module(self, module_name: String) -> Self {
        match self {
            ProgramSrcElement::Import(_) => self,
            ProgramSrcElement::Function(mut e) => {
                e.header.module = Some(module_name);
                ProgramSrcElement::Function(e)
            }
        }
    }
}


#[derive(Debug, PartialEq, Clone)]
pub enum ExprNode {
    Integer(i64),
    Float(f64),
    Bool(bool),
    Ident(String),
    Op(Box<ExprNode>, Op, Box<ExprNode>),
    FnCall(String, Option<Vec<Box<ExprNode>>>),
    UnaryOp(UnaryOp, Box<ExprNode>),
    BlockExpr(Block),
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
    TypeSym(String),
}

impl ToString for TypeInfo {
    fn to_string(&self) -> String {
        match self {
            TypeInfo::Int => String::from("int"),
            TypeInfo::Float => String::from("float"),
            TypeInfo::Bool => String::from("bool"),
            TypeInfo::Unit => String::from("unit"),
            TypeInfo::TypeSym(sym) => format!(".{}", sym)
        }
    }
}


impl TypeInfo {
    pub fn is_can_convert_to(&self, target: &TypeInfo) -> bool {
        if self == target {
            true
        } else if self == &TypeInfo::Int && target == &TypeInfo::Float {
            true
        } else {
            false
        }
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

