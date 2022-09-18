
use crate::error::LexerError;

#[derive(Debug, Copy, Clone)]
pub struct TokPos {
    pub line: usize,
    pub pos: usize,
}

impl TokPos {
    pub fn from(lexer: &Lexer) -> Self {
        TokPos {
            line: lexer.line,
            pos: lexer.pos - lexer.line_begin_pos + 1,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Tok {
    TokInteger(i32, TokPos),
    TokOp(String, TokPos),
    TokLeftParenthesis(TokPos),
    TokRightParenthesis(TokPos),
    TokEOF(TokPos),
}

impl Tok {
    pub fn pos(&self) -> &TokPos {
        match self {
            Tok::TokInteger(_, pos) => pos,
            Tok::TokOp(_, pos) => pos,
            Tok::TokEOF(pos) => pos,
            Tok::TokLeftParenthesis(pos) => pos,
            Tok::TokRightParenthesis(pos) => pos,
        }
    }
}

pub struct Lexer {
    pub code: Vec<char>,
    pub tok: Result<Tok, LexerError>,
    pos: usize,

    line: usize,
    // for meta information, it will be used to create `TokPos`
    line_begin_pos: usize,

}

impl Lexer {
    pub fn new(code: &str) -> Self {
        let mut lexer = Lexer {
            pos: 0,
            tok: Ok(Tok::TokEOF(TokPos { line: 0, pos: 0 })),
            line: 1,
            line_begin_pos: 0,
            code: code.chars().collect::<Vec<_>>(),
        };
        lexer.advance();
        lexer
    }

    fn inc_line(&mut self) {
        self.line += 1;
        self.line_begin_pos = self.pos;
    }
    fn eat_space(&mut self) {
        if self.pos >= self.code.len() {
            return;
        }
        let mut ch = self.code[self.pos];
        while ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r' {
            if ch == '\n' {
                self.inc_line();
            }
            self.pos += 1;
            if self.pos < self.code.len() {
                ch = self.code[self.pos];
            } else {
                break;
            }
        }
    }

    pub fn get_current_line(&self) -> String {
        let mut begin = self.pos;
        if begin >= self.code.len() {
            begin = self.code.len() - 1;
        }
        while begin > 0 && (self.code[begin] != '\r' || self.code[begin] != '\n') {
            begin -= 1;
        }
        let mut end = self.pos;
        while end < self.code.len() && (self.code[end] != '\r' || self.code[end] != '\n') {
            end += 1;
        }
        (&self.code[begin..end]).iter().collect::<String>()
    }

    fn lex_number(&mut self) {
        if !self.code[self.pos].is_ascii_digit() {
            self.tok = Err(LexerError::new(None, self.line, self.line_begin_pos, self.get_current_line(), "Expect a number here".to_string()))
        }
        let begin = self.pos;
        self.pos += 1;

        while self.pos < self.code.len() && self.code[self.pos].is_ascii_digit() {
            self.pos += 1;
        }

        let number: i32 = (&self.code[begin..self.pos]).iter().collect::<String>().parse().unwrap();

        self.tok = Ok(Tok::TokInteger(number, TokPos::from(self)));
    }

    fn err_here(&self, msg: String) -> LexerError {
        LexerError::new(None, self.line, self.line_begin_pos, self.get_current_line(), msg)
    }

    fn lex_op(&mut self) {
        let ch = self.code[self.pos];
        if ch == '+' || ch == '-' || ch == '*' || ch == '/' || ch == '%' {
            self.tok = Ok(Tok::TokOp(ch.to_string(), TokPos::from(self)));
            self.pos += 1;
            return;
        } else {
            self.tok = Err(self.err_here("Expect a operator here".to_string()));
        }
    }

    fn lex_parenthesis(&mut self) {
        self.tok = match self.code[self.pos] {
            '(' => Ok(Tok::TokLeftParenthesis(TokPos::from(self))),
            ')' => Ok(Tok::TokRightParenthesis(TokPos::from(self))),
            _ => Err(self.err_here("Expect a parenthesis here".to_string()))
        };
        self.pos += 1;
    }

    fn lex_relational(&mut self) {
        self.tok = if self.pos + 1 < self.code.len() {
            match self.code[self.pos] {
                '=' => match self.code[self.pos + 1] {
                    '=' => {
                        self.pos += 2;
                        Ok(Tok::TokOp(String::from("=="), TokPos::from(self)))
                    }
                    _ => {
                        self.pos += 1;
                        Ok(Tok::TokOp(String::from("="), TokPos::from(self)))
                    }
                },
                '!' => match self.code[self.pos + 1] {
                    '=' => {
                        self.pos += 2;
                        Ok(Tok::TokOp(String::from("!="), TokPos::from(self)))
                    }
                    _ => {
                        self.pos += 1;
                        Ok(Tok::TokOp(String::from("!"), TokPos::from(self)))
                    }
                },
                '>' => match self.code[self.pos + 1] {
                    '=' => {
                        self.pos += 2;
                        Ok(Tok::TokOp(String::from(">="), TokPos::from(self)))
                    }
                    _ => {
                        self.pos += 1;
                        Ok(Tok::TokOp(String::from(">"), TokPos::from(self)))
                    }
                },
                '<' => match self.code[self.pos + 1] {
                    '=' => {
                        self.pos += 2;
                        Ok(Tok::TokOp(String::from("<="), TokPos::from(self)))
                    }
                    _ => {
                        self.pos += 1;
                        Ok(Tok::TokOp(String::from("<"), TokPos::from(self)))
                    }
                },
                _ => Err(self.err_here("Expect a relational op here".to_string()))
            }
        } else {
            match self.code[self.pos] {
                '=' => {
                    self.pos += 1;
                    Ok(Tok::TokOp(String::from("="), TokPos::from(self)))
                }
                '!' => {
                    self.pos += 1;
                    Ok(Tok::TokOp(String::from("!"), TokPos::from(self)))
                }
                '>' => {
                    self.pos += 1;
                    Ok(Tok::TokOp(String::from(">"), TokPos::from(self)))
                }
                '<' => {
                    self.pos += 1;
                    Ok(Tok::TokOp(String::from("<="), TokPos::from(self)))
                }
                _ => Err(self.err_here("Expect a relational op here".to_string()))
            }
        }
    }


    pub fn advance(&mut self) {
        self.eat_space();
        if self.pos >= self.code.len() {
            self.tok = Ok(Tok::TokEOF(TokPos::from(self)));
            return;
        }
        let ch = self.code[self.pos];
        if ch.is_ascii_digit() {
            self.lex_number();
            return;
        } else if ch == '+' || ch == '-' || ch == '*' || ch == '/' || ch == '%' {
            self.lex_op();
            return;
        } else if ch == '(' || ch == ')' {
            self.lex_parenthesis();
            return;
        } else if ch == '=' || ch == '<' || ch == '>' || ch == '!' {
            self.lex_relational();
            return;
        } else {
            self.tok = Err(self.err_here("Unrecognised token here".to_string()))
        }
    }
}