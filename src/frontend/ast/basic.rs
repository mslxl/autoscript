pub type StmtBlock = Vec<AstStmtNode>;

#[derive(Debug, PartialEq, Clone)]
pub enum AstStmtNode {
    ExprStmt(Box<AstExprNode>),
    RetStmt(Option<Box<AstExprNode>>),
    VarStmt(String, Option<TypeInfo>, bool, Box<AstExprNode>),
    WhileStmt(Box<AstExprNode>, StmtBlock)
}

pub type AccessedIdent = Vec<String>;

#[derive(Debug, PartialEq, Clone)]
pub enum AstExprNode {
    Integer(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Ident(AccessedIdent),
    Op(Box<AstExprNode>, Op, Box<AstExprNode>),
    FnCall(AccessedIdent, Option<Vec<Box<AstExprNode>>>),
    UnaryOp(UnaryOp, Box<AstExprNode>),
    BlockExpr(StmtBlock),
    AssignExpr(String, Box<AstExprNode>),
    IfExpr(Box<AstExprNode>, Box<AstExprNode>, Option<Box<AstExprNode>>),//last stmt is return value
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

