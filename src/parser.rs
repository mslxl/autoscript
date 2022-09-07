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
    /*
    S      -> add
    add    -> add + mul | add - mul | mul
    mul    -> mul * integer | mul / integer | integer
    */


    pub fn parse(&mut self) -> Option<ExprNode> {
        match self.lexer.tok {
            Tok::TokEOF => None,
            _ => Some(self.add())
        }
    }
    /*
    add -> add + mul | add - mul | mul

    add -> mul add'
    add'-> + mul add' | - mul add' | ""
     */
    fn add(&mut self) -> ExprNode {
        let mut left = self.mul();

        while let Tok::TokOp(ref op, _) = self.lexer.tok {
            if op == "+" || op == "-" {
                let op = op.clone();
                self.lexer.advance();
                let right = self.mul();
                left = ExprNode::Op(Box::new(left), op, Box::new(right));
            } else {
                break;
            }
        }
        left
    }

    /*
    mul -> mul * integer | mul / integer | integer

    mul -> integer mul'
    mul' -> * integer mul' | / integer mul' | ""
     */

    fn mul(&mut self) -> ExprNode {
        let mut left = self.integer();

        while let Tok::TokOp(ref op, _) = self.lexer.tok {
            if op == "*" || op == "/" {
                let op = op.clone();
                self.lexer.advance();
                let right = self.integer();
                left = ExprNode::Op(Box::new(left), op, Box::new(right));
            } else {
                break;
            }
        }
        left
    }

    fn integer(&mut self) -> ExprNode {
        if let Tok::TokInteger(integer, _) = self.lexer.tok {
            let integer = integer;
            self.lexer.advance();
            ExprNode::Integer(integer)
        } else {
            panic!("Error: expect integer in {:?}", self.lexer.tok);
        }
    }
}