use std::fmt::Debug;

pub trait TopLevelScopeDecl:Debug {
    fn get_name(&self) -> &str;
    fn get_type(&self) -> &str;
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
pub struct Func {
    loc: Loc,
    name: String,
    return_type: TypeRef,
    args: Vec<(String, TypeRef, Option<Box<dyn Expr>>)>,
    block: Box<ComposableExpr>,
}
impl Func {
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
impl TopLevelScopeDecl for Func {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_type(&self) -> &str {
        &self.return_type.0
    }
}

#[derive(Debug)]
pub struct FuncCallExpr(Loc, String, Vec<Box<dyn Expr>>);
impl Expr for FuncCallExpr{
    fn loc(&self) -> Loc {
        self.0
    }
}
impl FuncCallExpr{
    pub fn new(l:usize, r:usize, name:String, args:Vec<Box<dyn Expr>>)->Self{
        Self(Loc::new(l,r), name, args)
    }
}

#[derive(Debug)]
pub struct ReturnStmt(Loc,Option<Box<dyn Expr>>);
impl Expr for ReturnStmt{
    fn loc(&self) -> Loc {
        self.0
    }
}
impl ReturnStmt{
    pub fn new(l:usize, r:usize, e:Option<Box<dyn Expr>>)->Self{
        Self(Loc::new(l,r),e)
    }
}


#[derive(Debug)]
pub struct DeclVarExpr {
    loc: Loc,
    name: String,
    var_type: Option<TypeRef>,
    mutable: bool,
    value: Box<dyn Expr>,
}
impl Expr for DeclVarExpr {
    fn loc(&self) -> Loc {
        self.loc
    }
}
impl DeclVarExpr {
    pub fn new(
        l: usize,
        r: usize,
        name: String,
        var_type: Option<TypeRef>,
        mutable: bool,
        value: Box<dyn Expr>,
    ) -> Self {
        Self {
            loc: Loc::new(l, r),
            name,
            var_type,
            mutable,
            value,
        }
    }
}
impl TopLevelScopeDecl for DeclVarExpr {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_type(&self) -> &str {
        &self.var_type.as_ref().unwrap().0
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
