use std::str::Chars;

#[derive(Debug)]
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

#[derive(Debug)]
pub enum Tok {
    TokInteger(i32, TokPos),
    TokOp(String, TokPos),
    TokEOF,
}

pub struct Lexer {
    pub code: Vec<char>,
    pub tok: Tok,
    pos: usize,

    line: usize,
    // for meta information, it will be used to create `TokPos`
    line_begin_pos: usize,

}

impl Lexer {
    pub fn new(code: &str) -> Self {
        let mut lexer = Lexer {
            pos: 0,
            tok: Tok::TokEOF,
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

            ch = self.code[self.pos];
        }
    }

    fn lex_number(&mut self) {
        if !self.code[self.pos].is_ascii_digit() {
            panic!("{} is not a number", self.code[self.pos]);
        }
        let begin = self.pos;
        self.pos += 1;

        while self.pos < self.code.len() && self.code[self.pos].is_ascii_digit() {
            self.pos += 1;
        }

        let number: i32 = (&self.code[begin..self.pos]).iter().collect::<String>().parse().unwrap();

        self.tok = Tok::TokInteger(number, TokPos::from(self));
    }

    fn lex_op(&mut self) {
        let ch = self.code[self.pos];
        if ch == '+' || ch == '-' || ch == '*' || ch == '/' || ch == '%' {
            self.tok = Tok::TokOp(ch.to_string(), TokPos::from(self));
            self.pos += 1;
            return;
        } else {
            panic!("{} is not a operator", ch);
        }
    }


    pub fn advance(&mut self) {
        self.eat_space();
        if self.pos >= self.code.len() {
            self.tok = Tok::TokEOF;
            return;
        }
        let ch = self.code[self.pos];
        if ch.is_ascii_digit() {
            self.lex_number();
            return;
        } else if ch == '+' || ch == '-' || ch == '*' || ch == '/' || ch == '%' {
            self.lex_op();
            return;
        }
    }
}