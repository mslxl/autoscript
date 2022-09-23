use std::iter::Enumerate;
use std::ops::{Range, RangeFrom, RangeFull, RangeTo};
use nom::{InputIter, InputLength, InputTake, Needed, Slice};

#[derive(PartialEq, Debug, Clone)]
pub enum Tok {
    EOF,
    Int(i64),
    Float(f64),
    Bool(bool),
    Ident(String),
    // punctuations
    LParen,
    RParen,
    LBrace,
    RBrace,
    Semicolon,
    Colon,
    RightArrow,
    // keyword
    KwdFn,
    KwdRet,
    KwdVar,
    KwdVal,


    // operator
    Plus,
    Minus,
    Multiply,
    Divide,
    Rem,
    Assign,

    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    Not,
    // special
    InfixOp(String),
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Tokens<'a> {
    pub tok: &'a [Tok],
    pub start: usize,
    pub end: usize,
}

impl<'a> Tokens<'a> {
    pub fn new(vec: &'a [Tok]) -> Self {
        Tokens {
            tok: vec,
            start: 0,
            end: vec.len(),
        }
    }
}

impl<'a> InputLength for Tokens<'a> {
    #[inline]
    fn input_len(&self) -> usize {
        self.tok.len()
    }
}

impl<'a> InputTake for Tokens<'a> {
    #[inline]
    fn take(&self, count: usize) -> Self {
        Tokens {
            tok: &self.tok[0..count],
            start: 0,
            end: count,
        }
    }
    #[inline]
    fn take_split(&self, count: usize) -> (Self, Self) {
        let (prefix, suffix) = self.tok.split_at(count);
        let first = Tokens::new(prefix);
        let second = Tokens::new(suffix);
        (second, first)
    }
}

impl InputLength for Tok {
    #[inline]
    fn input_len(&self) -> usize {
        1
    }
}

impl<'a> Slice<Range<usize>> for Tokens<'a> {
    #[inline]
    fn slice(&self, range: Range<usize>) -> Self {
        Tokens {
            tok: self.tok.slice(range.clone()),
            start: self.start + range.start,
            end: self.start + range.end,
        }
    }
}

impl<'a> Slice<RangeTo<usize>> for Tokens<'a> {
    #[inline]
    fn slice(&self, range: RangeTo<usize>) -> Self {
        self.slice(0..range.end)
    }
}

impl<'a> Slice<RangeFrom<usize>> for Tokens<'a> {
    #[inline]
    fn slice(&self, range: RangeFrom<usize>) -> Self {
        self.slice(range.start..self.end - self.start)
    }
}

impl<'a> Slice<RangeFull> for Tokens<'a> {
    #[inline]
    fn slice(&self, _: RangeFull) -> Self {
        Tokens {
            tok: self.tok,
            start: self.start,
            end: self.end,
        }
    }
}

impl<'a> InputIter for Tokens<'a> {
    type Item = &'a Tok;
    type Iter = Enumerate<::std::slice::Iter<'a, Tok>>;
    type IterElem = ::std::slice::Iter<'a, Tok>;

    #[inline]
    fn iter_indices(&self) -> Self::Iter {
        self.tok.iter().enumerate()
    }

    #[inline]
    fn iter_elements(&self) -> Self::IterElem {
        self.tok.iter()
    }

    #[inline]
    fn position<P>(&self, predicate: P) -> Option<usize> where P: Fn(Self::Item) -> bool {
        self.tok.iter().position(predicate)
    }

    #[inline]
    fn slice_index(&self, count: usize) -> Result<usize, Needed> {
        if self.tok.len() >= count {
            Ok(count)
        }else{
            Err(Needed::Unknown)
        }
    }
}