use std::{collections::HashMap, fmt::Debug, hash::Hash};

#[derive(Debug)]
pub enum TopLevelScopeDecl {
    FuncDecl(Box<FuncDecl>),
    VarDecl(Box<VarDeclExpr>),
    Import(Box<ImportStmt>),
    ClassDecl(Box<ClassDecl>),
}
pub enum ClassLevelScopeDecl {
    FuncDecl(Box<FuncDecl>),
    VarDecl(Box<VarDeclExpr>),
}

pub trait Expr: Debug {
    fn loc(&self) -> Loc;
}

#[derive(Copy, Clone, Debug)] //TODO: remove debug derive
pub struct Loc {
    pub left: usize,
    pub right: usize,
}

impl Loc {
    pub fn new(l: usize, r: usize) -> Self {
        Loc { left: l, right: r }
    }

    pub fn merge(l: &Loc, r: &Loc) -> Self {
        Loc {
            left: l.left,
            right: r.right,
        }
    }

    pub fn map_l(&self, b: fn(l: usize) -> usize) -> Self {
        Loc {
            left: b(self.left),
            right: self.right,
        }
    }

    pub fn map_r(&self, b: fn(l: usize) -> usize) -> Self {
        Loc {
            left: self.left,
            right: b(self.right),
        }
    }
}

#[derive(Debug)]
pub struct TypeRef(pub String);
#[derive(Debug)]
pub struct FuncDecl {
    pub loc: Loc,
    pub name: String,
    pub return_type: TypeRef,
    pub args: Vec<(String, TypeRef, Option<Box<dyn Expr>>)>,
    pub block: Box<ComposableExpr>,
}
impl FuncDecl {
    pub fn new(
        l: usize,
        r: usize,
        name: String,
        return_type: String,
        args: Vec<(String, TypeRef, Option<Box<dyn Expr>>)>,
        block: Box<ComposableExpr>,
    ) -> Self {
        Self {
            loc: Loc::new(l, r),
            return_type: TypeRef(return_type),
            name,
            args,
            block,
        }
    }
}

#[derive(Debug)]
pub struct ClassDecl {
    pub loc: Loc,
    pub name: String,
    pub fields: HashMap<String, VarDeclExpr>,
    pub methods: HashMap<String, FuncDecl>,
}

impl ClassDecl {
    pub fn new(l: usize, r: usize, name: String, element: Vec<ClassLevelScopeDecl>) -> Self {
        let mut decl = Self {
            loc: Loc::new(l, r),
            name: name,
            fields: HashMap::new(),
            methods: HashMap::new(),
        };
        for i in element {
            match i {
                ClassLevelScopeDecl::FuncDecl(f) => {
                    decl.methods.insert(f.name.clone(), *f);
                }
                ClassLevelScopeDecl::VarDecl(v) => {
                    decl.fields.insert(v.name.clone(), *v);
                }
            }
        }
        decl
    }
}

#[derive(Debug)]
pub struct ImportStmt(Loc, String);
impl ImportStmt {
    pub fn new(l: usize, r: usize, s: String) -> Self {
        Self(Loc::new(l, r), s)
    }
}

#[derive(Debug)]
pub struct FuncCallExpr(Loc, Box<dyn Expr>, Vec<Box<dyn Expr>>);
impl Expr for FuncCallExpr {
    fn loc(&self) -> Loc {
        self.0
    }
}
impl FuncCallExpr {
    pub fn new(l: usize, r: usize, name: Box<dyn Expr>, args: Vec<Box<dyn Expr>>) -> Self {
        Self(Loc::new(l, r), name, args)
    }
}

#[derive(Debug)]
pub struct ReturnExpr(Loc, Option<Box<dyn Expr>>);
impl Expr for ReturnExpr {
    fn loc(&self) -> Loc {
        self.0
    }
}
impl ReturnExpr {
    pub fn new(l: usize, r: usize, e: Option<Box<dyn Expr>>) -> Self {
        Self(Loc::new(l, r), e)
    }
}

#[derive(Debug)]
pub struct VarDeclExpr {
    pub loc: Loc,
    pub name: String,
    pub var_type: Option<TypeRef>,
    pub mutable: bool,
    pub value: Option<Box<dyn Expr>>,
}
impl Expr for VarDeclExpr {
    fn loc(&self) -> Loc {
        self.loc
    }
}
impl VarDeclExpr {
    pub fn new_novalue(
        l: usize,
        r: usize,
        name: String,
        var_type: Option<TypeRef>,
        mutable: bool,
    ) -> Self {
        Self {
            loc: Loc::new(l, r),
            name,
            var_type,
            mutable,
            value: None,
        }
    }
    pub fn new(
        l: usize,
        r: usize,
        name: String,
        var_type: Option<TypeRef>,
        mutable: bool,
        value: Box<dyn Expr>,
    ) -> Self {
        Self {
            value: Some(value),
            ..Self::new_novalue(l, r, name, var_type, mutable)
        }
    }
}

#[derive(Debug)]
pub struct UnitExpr(Loc);
impl Expr for UnitExpr {
    fn loc(&self) -> Loc {
        self.0
    }
}

impl UnitExpr {
    pub fn new(loc: usize) -> Self {
        UnitExpr(Loc::new(loc, loc))
    }
}

#[derive(Debug)]
pub struct IntConstExpr(Loc, i64);
impl Expr for IntConstExpr {
    fn loc(&self) -> Loc {
        self.0
    }
}
impl IntConstExpr {
    pub fn new(l: usize, r: usize, value: i64) -> Self {
        Self(Loc::new(l, r), value)
    }
}

#[derive(Debug)]
pub struct FloatConstExpr(Loc, f64);
impl Expr for FloatConstExpr {
    fn loc(&self) -> Loc {
        self.0
    }
}
impl FloatConstExpr {
    pub fn new(l: usize, r: usize, value: f64) -> Self {
        Self(Loc::new(l, r), value)
    }
}

#[derive(Debug)]
pub struct BoolConstExpr(Loc, bool);
impl Expr for BoolConstExpr {
    fn loc(&self) -> Loc {
        self.0
    }
}

impl BoolConstExpr {
    pub fn new(l: usize, r: usize, value: bool) -> Self {
        BoolConstExpr(Loc::new(l, r), value)
    }
    pub fn new_true(l: usize, r: usize) -> Self {
        Self::new(l, r, true)
    }
    pub fn new_false(l: usize, r: usize) -> Self {
        Self::new(l, r, false)
    }
}

#[derive(Debug)]
pub struct IdentifierExpr(Loc, String);
impl Expr for IdentifierExpr {
    fn loc(&self) -> Loc {
        self.0
    }
}
impl IdentifierExpr {
    pub fn new(l: usize, r: usize, value: String) -> Self {
        Self(Loc::new(l, r), value)
    }
}

#[derive(Debug)]
pub struct StrConstExpr(Loc, String);
impl Expr for StrConstExpr {
    fn loc(&self) -> Loc {
        self.0
    }
}
impl StrConstExpr {
    pub fn new(l: usize, r: usize, value: String) -> Self {
        Self(Loc::new(l, r), value)
    }
}

#[derive(Debug)]
pub struct CharConstExpr(Loc, char);
impl Expr for CharConstExpr {
    fn loc(&self) -> Loc {
        self.0
    }
}
impl CharConstExpr {
    pub fn new(l: usize, r: usize, value: char) -> Self {
        Self(Loc::new(l, r), value)
    }
}

#[derive(Debug)]

pub struct  MemberAccessExpr {
    pub loc:Loc,
    pub parent: Box<dyn Expr>,
    pub target: String,
}

impl Expr for MemberAccessExpr {
    fn loc(&self) -> Loc {
        self.loc
    }
}

impl MemberAccessExpr {
    pub fn new(l: usize, r: usize, parent: Box<dyn Expr>, target: String) -> Self {
        Self {
            loc: Loc::new(l, r),
            parent: parent,
            target: target,
        }
    }
}

#[derive(Debug)]
pub struct OpExpr(Loc, Box<dyn Expr>, Opcode, Box<dyn Expr>);
impl Expr for OpExpr {
    fn loc(&self) -> Loc {
        self.0
    }
}
impl OpExpr {
    pub fn new(lhs: Box<dyn Expr>, rhs: Box<dyn Expr>, opcode: Opcode) -> Self {
        Self(Loc::merge(&lhs.loc(), &rhs.loc()), lhs, opcode, rhs)
    }
}

#[derive(Debug)]
pub struct TriOpExpr(
    Loc,
    (Box<dyn Expr>, Opcode),
    Box<dyn Expr>,
    (Opcode, Box<dyn Expr>),
);
impl Expr for TriOpExpr {
    fn loc(&self) -> Loc {
        self.0
    }
}
impl TriOpExpr {
    pub fn new(
        lhs: Box<dyn Expr>,
        lop: Opcode,
        mhs: Box<dyn Expr>,
        rop: Opcode,
        rhs: Box<dyn Expr>,
    ) -> Self {
        let loc = Loc::merge(&lhs.loc(), &rhs.loc());
        Self(loc, (lhs, lop), mhs, (rop, rhs))
    }
}

#[derive(Debug)]
pub struct UnaryOpExpr(Loc, UnaryOpcode, Box<dyn Expr>);
impl Expr for UnaryOpExpr {
    fn loc(&self) -> Loc {
        self.0
    }
}
impl UnaryOpExpr {
    pub fn new(value: Box<dyn Expr>, op: UnaryOpcode) -> Self {
        Self(value.loc().map_l(|x| x - 1), op, value)
    }
}

#[derive(Debug)]
pub struct ComposableExpr(Loc, Vec<Box<dyn Expr>>);
impl Expr for ComposableExpr {
    fn loc(&self) -> Loc {
        self.0
    }
}
impl ComposableExpr {
    pub fn new(l: usize, r: usize, seq: Vec<Box<dyn Expr>>) -> Self {
        ComposableExpr(Loc::new(l, r), seq)
    }
}

#[derive(Debug)]
pub struct IfExpr {
    pub loc: Loc,
    pub cond: Box<dyn Expr>,
    pub then: Box<dyn Expr>,
    pub el: Box<dyn Expr>,
}
impl Expr for IfExpr {
    fn loc(&self) -> Loc {
        self.loc
    }
}
impl IfExpr {
    pub fn new(cond: Box<dyn Expr>, then: Box<dyn Expr>, el: Box<dyn Expr>) -> Self {
        Self {
            loc: Loc::merge(&cond.loc(), &el.loc()),
            cond,
            then,
            el,
        }
    }
}

#[derive(Debug)]
pub struct WhileExpr {
    pub loc: Loc,
    pub cond: Box<dyn Expr>,
    pub then: Box<dyn Expr>,
}
impl Expr for WhileExpr {
    fn loc(&self) -> Loc {
        self.loc
    }
}
impl WhileExpr {
    pub fn new(cond: Box<dyn Expr>, then: Box<dyn Expr>) -> Self {
        Self {
            loc: Loc::merge(&cond.loc(), &then.loc()),
            cond,
            then,
        }
    }
}

#[derive(Debug, Clone)]
pub enum UnaryOpcode {
    Not,
    // !
    Neg,
    // -
    Pos,
    // +
    BitNot, // ~

    Inc,
    // ++
    Dec, // --
}

#[derive(Debug, Clone)]
pub enum Opcode {
    MemberAccess,
    // .
    IndexAccess, // []

    Lsh,
    // <<
    Rsh,
    // >>
    RshUnsigned,
    // <<<
    BitAnd,
    // &
    BitOr,
    // |
    BitXor, // ^

    Mul,
    // *
    Div,
    // /
    Mod,
    // %
    Pow,
    // **
    MulAssign,
    // *=
    DivAssign,
    // /=
    ModAssign, // %=

    Add,
    // +
    Sub,
    // -
    AddAssign,
    // +=
    SubAssign, // -=

    Eq,
    // ==
    Ne,
    // !=
    Lt,
    // <
    Gt,
    // >
    Le,
    // <=
    Ge, // >=

    And,
    // &&
    Or, // ||
}
