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
        Loc { left: l.left, right: r.right }
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

#[derive(Debug, PartialEq, Eq)]
pub enum TypeRef {
    Int,
    Float,
    Bool,
    Str,
    Char,
    Complex(String),
    Unit,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Unit,
    Int(i64, Loc),
    Float(f64, Loc),
    Bool(bool, Loc),
    Identifier(String, Loc),
    Str(String, Loc),
    Char(char, Loc),
    OpExpr(Loc, Box<Expr>, Opcode, Box<Expr>),
    UnaryOpExpr(Loc, UnaryOpcode, Box<Expr>),
    ComposableExpr(Loc, Vec<Box<Expr>>),
    IfExpr {
        loc: Loc,
        cond: Box<Expr>,
        then: Box<Expr>,
        el: Box<Expr>,
    },
    WhileExpr {
        loc: Loc,
        cond: Box<Expr>,
        then: Box<Expr>,
    },
    RepeatExpr {
        loc: Loc,
        cond: Box<Expr>,
        then: Box<Expr>,
    },
    LoopExpr(Loc, Box<Expr>),
}

impl Expr {
    pub fn loc(&self) -> Loc {
        match self {
            Expr::Int(_, l) => *l,
            Expr::Float(_, l) => *l,
            Expr::Bool(_, l) => *l,
            Expr::Identifier(_, l) => *l,
            Expr::Str(_, l) => *l,
            Expr::Char(_, l) => *l,
            Expr::OpExpr(l, _, _, _) => *l,
            Expr::UnaryOpExpr(l, _, _) => *l,
            Expr::IfExpr { loc: l, .. } => *l,
            Expr::ComposableExpr(l, _) => *l,
            Expr::Unit => panic!("Expr::Unit has no location context"),
            Expr::WhileExpr { loc: l, .. } => *l,
            Expr::RepeatExpr { loc:l,.. } => *l,
            Expr::LoopExpr(l, _) => *l
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
    IndexAccess,  // []

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
    BitXor,      // ^

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
    Lt,
    // <
    Gt,
    // >
    Le,
    // <=
    Ge, // >=

    And,
    // &&
    Or,  // ||
}
