use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use crate::Tok;

#[derive(Debug,Clone)]
pub struct LexerError {
    file: Option<String>,
    line: usize,
    pos: usize,
    code: String,
    msg: String,
}

impl Display for LexerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let cursor = "^ ";
        write!(f, "{file}:{line}:{pos}\n\t{code}\n\t{cursor:>pos$}^{msg}",
               file = self.file.as_ref().unwrap_or(&"[Internal]".to_string()),
               line = self.line,
               pos = self.pos,
               code = self.code,
               msg = self.msg
        )
    }
}

impl Error for LexerError {}

impl LexerError {
    pub fn new(file: Option<String>, line: usize, pos: usize, code: String, msg: String) -> Self {
        LexerError {
            file,
            line,
            pos,
            code,
            msg,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParseError {
    file: Option<String>,
    line: usize,
    pos: usize,
    code: String,
    actual: Tok,
    expect: Option<Tok>
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let cursor = "^ ";
        let error_msg =  match &self.expect {
            Some(expectTok) =>
                format!("Unexpected token, expect {:?}, actual {:?}", expectTok, self.actual),
            None =>
                format!("Unexpected token: {actual:?}", actual = self.actual)
        };

        write!(f, "ParserError in {file}:{line}:{pos}\n\t{code}\n\t{cursor:>pos$}{error_msg}",
               file = self.file.as_ref().unwrap_or(&"[Internal]".to_string()),
               line = self.line,
               pos = self.pos,
               code = self.code
        )
    }
}

impl Error for ParseError{}

impl ParseError{
    pub fn new(file:Option<String>, line:usize, pos:usize, code:String, expect:Option<Tok>, actual:Tok) -> Self{
        ParseError{
            file,
            line,
            pos,
            code,
            expect,
            actual,
        }
    }
}