use std::error::Error;
use crate::error::ParseError;
use crate::ast::ExprNode;
use crate::lexer::Lexer;
use crate::Tok;

pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Parser {
            lexer
        }
    }

    fn get_current_line(&self) -> String {
        self.lexer.get_current_line()
    }

    fn error_unexpect(&self) -> Box<dyn Error> {
        match &self.lexer.tok {
            Err(e) => Box::new((*e).clone()),
            Ok(tok) => {
                let pos = tok.pos();
                let tok = (*tok).clone();
                let err = ParseError::new(None,
                                          pos.line,
                                          pos.pos,
                                          self.get_current_line(),
                                          None, tok);
                Box::new(err)
            }
        }
    }

    fn error_expect_unsatisfying(&self, expect: String) -> Box<dyn Error> {
        match &self.lexer.tok {
            Err(e) => Box::new((*e).clone()),
            Ok(tok) => {
                let pos = tok.pos();
                let tok = (*tok).clone();
                let err = ParseError::new(Some(expect),
                                          pos.line,
                                          pos.pos,
                                          self.get_current_line(),
                                          None, tok);
                Box::new(err)
            }
        }
    }


    pub fn parse(&mut self) -> Result<ExprNode, Box<dyn Error>> {
        match self.lexer.tok.as_ref().unwrap() {
            Tok::TokEOF(_) => Err(self.error_unexpect()),
            _ => Ok(self.expr()?)
        }
    }

    fn expr(&mut self) -> Result<ExprNode, Box<dyn Error>> {
        self.equality()
    }
    fn equality(&mut self) -> Result<ExprNode, Box<dyn Error>>{
        let left = self.relational()?;
        let op = match self.lexer.tok.as_ref() {
            Err(e) => {
                let err:Box<dyn Error> = Box::new((*e).clone());
                Err(err)
            },
            Ok(op) => if let Tok::TokOp(op, _) = op {
                let op: &str = op;
                match op {
                    "==" => Ok("=="),
                    "!=" => Ok("!="),
                    _ => return Ok(left)
                }
            }else {
                return Ok(left);
            }
        }?;
        self.lexer.advance();
        let right = self.relational()?;
        Ok(ExprNode::Op(Box::new(left), String::from(op), Box::new(right)))
    }

    fn relational(&mut self) -> Result<ExprNode, Box<dyn Error>>{
        let left = self.add()?;
        let op = match self.lexer.tok.as_ref() {
            Err(e) => {
                let err:Box<dyn Error> = Box::new((*e).clone());
                Err(err)
            },
            Ok(op) => if let Tok::TokOp(op, _) = op {
                let op: &str = op;
                match op {
                    "<" => Ok("<"),
                    ">" => Ok(">"),
                    ">=" => Ok(">="),
                    "<=" => Ok("<="),
                    _ => return Ok(left)
                }
            }else {
                return Ok(left)
            }
        }?;
        self.lexer.advance();
        let right = self.add()?;
        Ok(ExprNode::Op(Box::new(left),String::from(op), Box::new(right)))
    }

    fn add(&mut self) -> Result<ExprNode, Box<dyn Error>> {
        let mut left = self.mul()?;

        while let Tok::TokOp(ref op, _) = self.lexer.tok.as_ref().unwrap() {
            if op == "+" || op == "-" {
                let op = op.clone();
                self.lexer.advance();
                let right = self.mul()?;
                left = ExprNode::Op(Box::new(left), op, Box::new(right));
            } else {
                break;
            }
        }
        Ok(left)
    }

    fn mul(&mut self) -> Result<ExprNode, Box<dyn Error>> {
        let mut left = self.unary()?;

        while let Tok::TokOp(ref op, _) = self.lexer.tok.as_ref().unwrap() {
            if op == "*" || op == "/" {
                let op = op.clone();
                self.lexer.advance();
                let right = self.unary()?;
                left = ExprNode::Op(Box::new(left), op, Box::new(right));
            } else {
                break;
            }
        }
        Ok(left)
    }

    fn unary(&mut self) -> Result<ExprNode, Box<dyn Error>> {
        match self.lexer.tok.as_ref() {
            Err(e) => Err(Box::new((*e).clone())),
            Ok(tok) => {
                if let Tok::TokOp(op, _) = tok {
                    if op == "+" || op == "-" {
                        let op = op.clone();
                        self.lexer.advance();
                        let expr = self.unary()?;
                        Ok(ExprNode::UnaryOp(op, Box::new(expr)))
                    }else{
                        self.primary()
                    }
                }else{
                    self.primary()
                }
            }
        }
    }

    fn primary(&mut self) -> Result<ExprNode, Box<dyn Error>> {
        if let Ok(Tok::TokLeftParenthesis(_)) = self.lexer.tok.as_ref() {
            self.lexer.advance();
            let expr = self.expr()?;
            if let Ok(Tok::TokRightParenthesis(_)) = self.lexer.tok {} else {
                self.error_expect_unsatisfying(")".to_string());
            }
            Ok(expr)
        } else {
            self.num()
        }
    }

    fn num(&mut self) -> Result<ExprNode, Box<dyn Error>> {
        self.integer()
    }


    fn integer(&mut self) -> Result<ExprNode, Box<dyn Error>> {
        if let Ok(tok) = self.lexer.tok.as_ref() {
            if let Tok::TokInteger(integer, _) = tok {
                let integer = *integer;
                self.lexer.advance();
                Ok(ExprNode::Integer(integer))
            } else {
                Err(self.error_unexpect())
            }
        } else {
            Err(Box::new(self.lexer.tok.as_ref().unwrap_err().clone()))
        }
    }
}