use nom::branch::alt;
use nom::bytes::complete::take;
use nom::combinator::{map, opt, verify};
use nom::Err;
use nom::error::{Error, ErrorKind};
use nom::IResult;
use nom::multi::many0;
use nom::sequence::{delimited, pair, preceded, terminated, tuple};

use crate::frontend::ast::basic::{AccessedIdent, AstExprNode, AstStmtNode, Op, StmtBlock, TypeInfo, UnaryOp};
use crate::frontend::ast::element::{AstProgramFunctionImplElement, ProgramClassElement, ProgramElement};
use crate::frontend::ast::func::{FunctionBasicInfo, FunctionOrigin};
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
tag_token!(and_tag, Tok::And);
tag_token!(or_tag, Tok::Or);

tag_token!(lparen_tag, Tok::LParen);
tag_token!(rparen_tag, Tok::RParen);
tag_token!(lbrace_tag, Tok::LBrace);
tag_token!(rbrace_tag, Tok::RBrace);
tag_token!(assign_tag, Tok::Assign);
tag_token!(semicolon_tag, Tok::Semicolon);
tag_token!(colon_tag, Tok::Colon);
tag_token!(rarrow_tag, Tok::RightArrow);
tag_token!(comma_tag, Tok::Comma);
tag_token!(not_tag, Tok::Not);
tag_token!(dot_tag, Tok::Dot);

tag_token!(fn_kwd_tag, Tok::KwdFn);
tag_token!(ret_kwd_tag, Tok::KwdRet);

tag_token!(var_kwd_tag, Tok::KwdVal);
tag_token!(val_kwd_tag, Tok::KwdVar);

tag_token!(if_kwd_tag, Tok::KwdIf);
tag_token!(elif_kwd_tag, Tok::KwdElif);
tag_token!(else_kwd_tag, Tok::KwdElse);
tag_token!(while_kwd_tag, Tok::KwdWhile);

tag_token!(import_kwd_tag, Tok::KwdImport);
tag_token!(class_kwd_tag, Tok::KwdClass);

fn parse_ident(input: Tokens) -> IResult<Tokens, String> {
    let (i1, t1) = take(1usize)(input)?;
    if t1.tok.is_empty() {
        Err(Err::Error(Error::new(input, ErrorKind::Tag)))
    } else {
        match t1.tok[0].clone() {
            Tok::Ident(name) => Ok((i1, name)),
            _ => Err(Err::Error(Error::new(input, ErrorKind::Tag))),
        }
    }
}

fn parse_accessed_ident(input: Tokens) -> IResult<Tokens, AccessedIdent> {
    let (i1, (id, mut idents)) = pair(parse_ident, many0(preceded(dot_tag, parse_ident)))(input)?;
    idents.insert(0, id);
    Ok((i1, idents))
}

fn parse_ident_expr(input: Tokens) -> IResult<Tokens, Box<AstExprNode>> {
    map(parse_accessed_ident, |tok| Box::new(AstExprNode::Ident(tok)))(input)
}

fn parse_num(input: Tokens) -> IResult<Tokens, Box<AstExprNode>> {
    let (i1, t1) = take(1usize)(input)?;
    if t1.tok.is_empty() {
        Err(Err::Error(Error::new(input, ErrorKind::Tag)))
    } else {
        match t1.tok.first().unwrap() {
            Tok::Int(num) => Ok((i1, Box::new(AstExprNode::Integer(*num)))),
            Tok::Float(num) => Ok((i1, Box::new(AstExprNode::Float(*num)))),
            _ => Err(Err::Error(Error::new(input, ErrorKind::Tag)))
        }
    }
}

fn parse_bool(input: Tokens) -> IResult<Tokens, Box<AstExprNode>> {
    let (i1, t1) = take(1usize)(input)?;
    if t1.tok.is_empty() {
        Err(Err::Error(Error::new(input, ErrorKind::Tag)))
    } else {
        match t1.tok.first().unwrap() {
            Tok::Bool(b) => Ok((i1, Box::new(AstExprNode::Bool(*b)))),
            _ => Err(Err::Error(Error::new(input, ErrorKind::Tag)))
        }
    }
}


fn parse_string(input: Tokens) -> IResult<Tokens, Box<AstExprNode>> {
    let (i1, t1) = take(1usize)(input)?;
    if t1.tok.is_empty() {
        Err(Err::Error(Error::new(input, ErrorKind::Tag)))
    } else {
        match t1.tok.first().unwrap() {
            Tok::String(s) => Ok((i1, Box::new(AstExprNode::String(s.clone())))),
            _ => Err(Err::Error(Error::new(input, ErrorKind::Tag)))
        }
    }
}


fn parse_primary(input: Tokens) -> IResult<Tokens, Box<AstExprNode>> {
    let fst_match = tuple((lparen_tag, parse_expr, rparen_tag))(input);
    if fst_match.is_ok() {
        let (i1, (_, expr, _)) = fst_match.unwrap();
        Ok((i1, expr))
    } else {
        alt((parse_fn_call, parse_assign_expr, parse_num, parse_bool, parse_string, parse_if_expr, parse_ident_expr))(input)
    }
}


fn parse_unary(input: Tokens) -> IResult<Tokens, Box<AstExprNode>> {
    let fst_match = pair(alt((plus_tag, minus_tag, not_tag)), parse_unary)(input);
    if fst_match.is_ok() {
        let (i1, (tokens, expr)) = fst_match.unwrap();
        let op = match tokens.tok.first().unwrap() {
            Tok::Plus => UnaryOp::Plus,
            Tok::Minus => UnaryOp::Minus,
            Tok::Not => UnaryOp::Not,
            _ => unreachable!()
        };
        Ok((i1, Box::new(AstExprNode::UnaryOp(op, expr))))
    } else {
        parse_primary(input)
    }
}

fn parse_mul(input: Tokens) -> IResult<Tokens, Box<AstExprNode>> {
    let (i1, (mut lhs, seq)) = pair(parse_unary, many0(pair(alt((mul_tag, div_tag, rem_tag)), parse_unary)))(input)?;
    for (tokens, rhs) in seq {
        let op = match tokens.tok.first().unwrap() {
            Tok::Multiply => Op::Mul,
            Tok::Divide => Op::Div,
            Tok::Rem => Op::Rem,
            _ => unreachable!()
        };
        lhs = Box::new(AstExprNode::Op(lhs, op, rhs))
    }
    Ok((i1, lhs))
}

fn parse_add(input: Tokens) -> IResult<Tokens, Box<AstExprNode>> {
    let (i1, (mut lhs, seq)) = pair(parse_mul, many0(pair(alt((plus_tag, minus_tag)), parse_mul)))(input)?;
    for (tokens, rhs) in seq {
        let op = match tokens.tok.first().unwrap() {
            Tok::Plus => Op::Add,
            Tok::Minus => Op::Sub,
            _ => unreachable!()
        };
        lhs = Box::new(AstExprNode::Op(lhs, op, rhs))
    }
    Ok((i1, lhs))
}

fn parse_relational(input: Tokens) -> IResult<Tokens, Box<AstExprNode>> {
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
        Ok((i1, Box::new(AstExprNode::Op(lhs, op, rhs))))
    } else {
        parse_add(input)
    }
}

fn parse_equality(input: Tokens) -> IResult<Tokens, Box<AstExprNode>> {
    let fst_match = tuple((parse_relational, alt((eq_tag, ne_tag)), parse_relational))(input);
    if fst_match.is_ok() {
        let (i1, (lhs, tokens, rhs)) = fst_match.unwrap();
        let op = match tokens.tok.first().unwrap() {
            Tok::Ne => Op::Ne,
            Tok::Eq => Op::Eq,
            _ => unreachable!()
        };
        Ok((i1, Box::new(AstExprNode::Op(lhs, op, rhs))))
    } else {
        parse_relational(input)
    }
}

fn parse_logic(input: Tokens) -> IResult<Tokens, Box<AstExprNode>> {
    let (i1, (mut lhs, seq)) = pair(parse_equality, many0(pair(alt((and_tag, or_tag)), parse_equality)))(input)?;
    for (tokens, rhs) in seq {
        let op = match tokens.tok.first().unwrap() {
            Tok::And => Op::And,
            Tok::Or => Op::Or,
            _ => unreachable!()
        };
        lhs = Box::new(AstExprNode::Op(lhs, op, rhs))
    }
    Ok((i1, lhs))
}

fn parse_comma_expr(input: Tokens) -> IResult<Tokens, Vec<Box<AstExprNode>>> {
    let (i1, (expr, mut exprs)) = pair(parse_expr, many0(preceded(comma_tag, parse_expr)))(input)?;
    exprs.insert(0, expr);
    Ok((i1, exprs))
}

fn parse_fn_call(input: Tokens) -> IResult<Tokens, Box<AstExprNode>> {
    let (i1, (fn_name, _, args, _)) = tuple((parse_accessed_ident, lparen_tag, opt(parse_comma_expr), rparen_tag))(input)?;
    let expr = Box::new(AstExprNode::FnCall(fn_name, args));
    Ok((i1, expr))
}

fn parse_expr(input: Tokens) -> IResult<Tokens, Box<AstExprNode>> {
    parse_logic(input)
}

fn parse_if_expr(input: Tokens) -> IResult<Tokens, Box<AstExprNode>> {
    fn parse_else(input: Tokens) -> IResult<Tokens, Box<AstExprNode>> {
        preceded(
            else_kwd_tag,
            map(
                parse_block_stmt,
                |x| Box::new(AstExprNode::BlockExpr(x))))(input)
    }
    fn parse_elif(input: Tokens) -> IResult<Tokens, Box<AstExprNode>> {
        let (i1, (_, cond, code)) = tuple(
            (elif_kwd_tag,
             parse_expr,
             parse_block_stmt))(input)?;
        let ret = opt(alt((parse_elif, parse_else)))(i1);
        if let Ok((tok, els)) = ret {
            Ok((tok, Box::new(
                AstExprNode::IfExpr(
                    cond,
                    Box::new(AstExprNode::BlockExpr(code)),
                    els))))
        } else {
            Ok((i1, Box::new(
                AstExprNode::IfExpr(
                    cond,
                    Box::new(AstExprNode::BlockExpr(code)),
                    None))))
        }
    }

    let (i1, (_, cond, code, els)) = tuple((if_kwd_tag, parse_expr, parse_block_stmt, opt(alt((parse_elif, parse_else)))))(input)?;
    let expr = Box::new(AstExprNode::IfExpr(cond, Box::new(AstExprNode::BlockExpr(code)), els));
    Ok((i1, expr))
}

fn parse_expr_stmt(input: Tokens) -> IResult<Tokens, AstStmtNode> {
    map(terminated(parse_expr, opt(semicolon_tag)), |expr| {
        AstStmtNode::ExprStmt(expr)
    })(input)
}

fn parse_var_stmt(input: Tokens) -> IResult<Tokens, AstStmtNode> {
    let (i1, (kwd, id, ty, _, expr, _)) = tuple((
        alt((val_kwd_tag, var_kwd_tag)),
        parse_ident,
        opt(preceded(colon_tag, parse_ident)),
        assign_tag,
        parse_expr,
        opt(semicolon_tag)))(input)?;
    let is_const = kwd.tok.first().unwrap() == &Tok::KwdVal;
    let stmt = AstStmtNode::VarStmt(id, ty.map(TypeInfo::from), is_const, expr);
    Ok((i1, stmt))
}

fn parse_assign_expr(input: Tokens) -> IResult<Tokens, Box<AstExprNode>> {
    let (i1, (id, _, expr)) = tuple((parse_ident, assign_tag, parse_expr))(input)?;
    let expr = Box::new(AstExprNode::AssignExpr(id, expr));
    Ok((i1, expr))
}


fn parse_ret_stmt(input: Tokens) -> IResult<Tokens, AstStmtNode> {
    map(delimited(ret_kwd_tag, opt(parse_expr), opt(semicolon_tag)), |expr| {
        AstStmtNode::RetStmt(expr)
    })(input)
}

fn parse_while_stmt(input: Tokens) -> IResult<Tokens, AstStmtNode> {
    let (i1, (cond, stmt)) = preceded(while_kwd_tag, pair(parse_expr, parse_block_stmt))(input)?;

    Ok((i1, AstStmtNode::WhileStmt(cond, stmt)))
}

fn parse_stmt(input: Tokens) -> IResult<Tokens, AstStmtNode> {
    alt((
        parse_ret_stmt,
        parse_var_stmt,
        parse_while_stmt,
        parse_expr_stmt, ))(input)
}

fn parse_block_stmt(input: Tokens) -> IResult<Tokens, StmtBlock> {
    delimited(lbrace_tag, many0(parse_stmt), rbrace_tag)(input)
}

fn parse_func_params(input: Tokens) -> IResult<Tokens, Vec<(String, TypeInfo)>> {
    fn parse_func_param_item(input: Tokens) -> IResult<Tokens, (String, TypeInfo)> {
        map(tuple((parse_ident, colon_tag, parse_ident)), |item| (item.0, TypeInfo::from(item.2.as_str())))(input)
    }
    let (i1, (param, mut params)) = pair(parse_func_param_item, many0(preceded(comma_tag, parse_func_param_item)))(input)?;
    params.insert(0, param);
    Ok((i1, params))
}

fn parse_func(input: Tokens) -> IResult<Tokens, ProgramElement> {
    let (i1, (_, id, _, params, _, ret_value, block)) = tuple((
        fn_kwd_tag,
        parse_ident,
        lparen_tag,
        opt(parse_func_params),
        rparen_tag,
        opt(pair(rarrow_tag, parse_ident)),
        parse_block_stmt))(input)?;
    let func = ProgramElement::Function(AstProgramFunctionImplElement {
        header: FunctionBasicInfo {
            name: id,
            param: params,
            module: None,
            ret: match ret_value {
                None => None,
                Some((_, id)) => Some(TypeInfo::from(id.as_str())),
            },
            origin: FunctionOrigin::Source,
        },
        block,
    });
    Ok((i1, func))
}

fn parse_class(input: Tokens) -> IResult<Tokens, ProgramElement>{
    let (i1, (name, _, _)) = preceded(class_kwd_tag, tuple((parse_ident, lbrace_tag, rbrace_tag)))(input)?;
    let class = ProgramClassElement{
        name,
        module: String::from("")
    };
    Ok((i1, ProgramElement::Class(class)))
}

fn parse_import(input: Tokens) -> IResult<Tokens, ProgramElement> {
    let (i1, (_, module_name, _)) = tuple((import_kwd_tag, parse_ident, opt(semicolon_tag)))(input)?;
    Ok((i1, ProgramElement::Import(module_name)))
}

fn parse_program(input: Tokens) -> IResult<Tokens, ProgramElement> {
    alt((parse_func, parse_import, parse_class))(input)
}

pub struct Parser;

impl Parser {
    pub fn parse(tokens: Tokens, module_name: &str) -> Vec<ProgramElement> {
        let (i1, program) = many0(parse_program)(tokens).unwrap();
        assert!(i1.tok.is_empty());
        program.into_iter()
            .map(|e| e.set_module(module_name.to_string()))
            .collect()
    }
}