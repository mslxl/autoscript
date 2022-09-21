use nom::branch::alt;
use nom::bytes::complete::take;
use nom::combinator::{map, opt, verify};
use nom::Err;
use nom::error::{Error, ErrorKind};
use nom::IResult;
use nom::multi::many0;
use nom::sequence::{pair, tuple};
use crate::frontend::ast::{ExprNode, Op, UnaryOp};
use crate::frontend::tok::{Tok, Tokens};
macro_rules! tag_token (
  ($func_name:ident, $tag:expr) => (
      fn $func_name (tokens: Tokens) -> IResult<Tokens, Tokens> {
          verify(take(1usize), |t:&Tokens| t.tok[0] == $tag)(tokens)
      }
  )
);

tag_token!(plus_tag, Tok::Plus);
tag_token!(minus_tag, Tok::Minus);
tag_token!(mul_tag, Tok::Multiply);
tag_token!(div_tag, Tok::Divide);
tag_token!(rem_tag, Tok::Rem);
tag_token!(le_tag, Tok::Le);
tag_token!(lt_tag, Tok::Lt);
tag_token!(ge_tag, Tok::Ge);
tag_token!(gt_tag, Tok::Gt);
tag_token!(eq_tag, Tok::Eq);
tag_token!(ne_tag, Tok::Ne);

tag_token!(lparen_tag, Tok::LParen);
tag_token!(rparen_tag, Tok::RParen);

fn parse_num(input: Tokens) -> IResult<Tokens, Box<ExprNode>> {
    let (i1, t1) = take(1usize)(input)?;
    if t1.tok.is_empty() {
        Err(Err::Error(Error::new(input, ErrorKind::Tag)))
    } else {
        match t1.tok.first().unwrap() {
            Tok::Int(num) => Ok((i1, Box::new(ExprNode::Integer(*num)))),
            Tok::Float(num) => Ok((i1, Box::new(ExprNode::Float(*num)))),
            _ => Err(Err::Error(Error::new(input, ErrorKind::Tag)))
        }
    }
}

fn parse_primary(input: Tokens) -> IResult<Tokens, Box<ExprNode>> {
    let fst_match = tuple((lparen_tag, parse_expr, rparen_tag))(input);
    if fst_match.is_ok() {
        let (i1, (_, expr, _)) = fst_match.unwrap();
        Ok((i1, expr))
    } else {
        parse_num(input)
    }
}


fn parse_unary(input: Tokens) -> IResult<Tokens, Box<ExprNode>> {
    let fst_match = pair((alt((plus_tag, minus_tag))), parse_unary)(input);
    if fst_match.is_ok() {
        let (i1, (tokens, expr)) = fst_match.unwrap();
        let op = match tokens.tok.first().unwrap() {
            Tok::Plus => UnaryOp::Plus,
            Tok::Minus => UnaryOp::Minus,
            _ => unreachable!()
        };
        Ok((i1, Box::new(ExprNode::UnaryOp(op, expr))))
    } else {
        parse_primary(input)
    }
}

fn parse_mul(input: Tokens) -> IResult<Tokens, Box<ExprNode>> {
    let (i1, (mut lhs, seq)) = pair(parse_unary, many0(pair(alt((mul_tag, div_tag, rem_tag)), parse_unary)))(input)?;
    for (tokens, rhs) in seq {
        let op = match tokens.tok.first().unwrap() {
            Tok::Multiply => Op::Mul,
            Tok::Divide => Op::Div,
            Tok::Rem => Op::Rem,
            _ => unreachable!()
        };
        lhs = Box::new(ExprNode::Op(lhs, op, rhs))
    }
    Ok((i1, lhs))
}

fn parse_add(input: Tokens) -> IResult<Tokens, Box<ExprNode>> {
    let (i1, (mut lhs, seq)) = pair(parse_mul, many0(pair(alt((plus_tag, minus_tag)), parse_mul)))(input)?;
    for (tokens, rhs) in seq {
        let op = match tokens.tok.first().unwrap() {
            Tok::Plus => Op::Add,
            Tok::Minus => Op::Sub,
            _ => unreachable!()
        };
        lhs = Box::new(ExprNode::Op(lhs, op, rhs))
    }
    Ok((i1, lhs))
}

fn parse_relational(input: Tokens) -> IResult<Tokens, Box<ExprNode>> {
    let fst_match = tuple((parse_add, alt((le_tag, ge_tag, lt_tag, gt_tag)), parse_add))(input);
    if fst_match.is_ok() {
        let (i1, (lhs, tokens, rhs)) = fst_match.unwrap();
        let op = match tokens.tok.first().unwrap() {
            Tok::Lt => Op::Lt,
            Tok::Le => Op::Le,
            Tok::Gt => Op::Gt,
            Tok::Ge => Op::Ge,
            _ => unreachable!()
        };
        Ok((i1, Box::new(ExprNode::Op(lhs, op, rhs))))
    } else {
        parse_add(input)
    }
}

fn parse_equality(input: Tokens) -> IResult<Tokens, Box<ExprNode>> {
    let fst_match = tuple((parse_relational, alt((eq_tag, ne_tag)), parse_relational))(input);
    if fst_match.is_ok() {
        let (i1, (lhs, tokens, rhs)) = fst_match.unwrap();
        let op = match tokens.tok.first().unwrap() {
            Tok::Ne => Op::Ne,
            Tok::Eq => Op::Eq,
            _ => unreachable!()
        };
        Ok((i1, Box::new(ExprNode::Op(lhs, op, rhs))))
    } else {
        parse_relational(input)
    }
}

fn parse_expr(input: Tokens) -> IResult<Tokens, Box<ExprNode>> {
    parse_equality(input)
}

pub struct Parser;

impl Parser {
    pub fn parse(tokens: Tokens) -> IResult<Tokens, Box<ExprNode>> {
        parse_expr(tokens)
    }
}

mod tests {
    use std::ops::Not;
    use super::*;
    use crate::frontend::lexer::Lexer;

    fn assert_expr(input: &str, expr_expect: ExprNode) {
        let input = input.as_bytes();
        let (remain, tok) = Lexer::lex_tokens(input).unwrap();
        assert_eq!(remain.len(), 0);
        let tokens = Tokens::new(&tok);
        let (remain, expr) = Parser::parse(tokens).unwrap();

        if remain.tok.is_empty().not() {
            println!("{:?}", remain);
            assert_eq!(remain.tok.len(), 0);
        }
        assert_eq!(expr, Box::new(expr_expect))
    }

    #[test]
    fn test_basic_add() {
        assert_expr("1+1",
                    ExprNode::Op(
                        Box::new(ExprNode::Integer(1)),
                        Op::Add,
                        Box::new(ExprNode::Integer(1))
                    ));
    }
}


