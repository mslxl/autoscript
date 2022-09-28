use std::str::FromStr;
use nom::branch::alt;
use nom::IResult;
use nom::combinator::{map, map_res, opt, recognize};
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, alphanumeric1, char, digit1, multispace0};
use nom::multi::many0;
use nom::sequence::{delimited, pair, tuple};

use crate::frontend::tok::*;
macro_rules! literal_lex {
     ($func_name: ident, $tag_string: literal, $output_token: expr) => {
         fn $func_name<'a>(s: &'a [u8]) -> IResult<&[u8], Tok> {
             map(tag($tag_string), |_| $output_token)(s)
         }
     }
}

literal_lex!(eq_op, "==", Tok::Eq);
literal_lex!(ne_op, "!=", Tok::Ne);
literal_lex!(assign_op, "=", Tok::Assign);
literal_lex!(plus_op, "+", Tok::Plus);
literal_lex!(minus_op, "-", Tok::Minus);
literal_lex!(multiply_op, "*", Tok::Multiply);
literal_lex!(divide_op, "/", Tok::Divide);
literal_lex!(rem_op, "%", Tok::Rem);
literal_lex!(not_op, "!", Tok::Not);
literal_lex!(ge_op, ">=", Tok::Ge);
literal_lex!(le_op, "<=", Tok::Le);
literal_lex!(gt_op, ">", Tok::Gt);
literal_lex!(lt_op, "<", Tok::Lt);
literal_lex!(and_op, "&&", Tok::And);
literal_lex!(or_op, "||", Tok::Or);

fn lex_operator(input: &[u8]) -> IResult<&[u8], Tok> {
    alt((
        eq_op,
        ne_op,
        assign_op,
        plus_op,
        minus_op,
        multiply_op,
        divide_op,
        rem_op,
        not_op,
        ge_op,
        le_op,
        gt_op,
        lt_op,
        and_op,
        or_op
    ))(input)
}

literal_lex!(lparen_punctuation, "(", Tok::LParen);
literal_lex!(rparen_punctuation, ")", Tok::RParen);
literal_lex!(lbrace_punctuation, "{", Tok::LBrace);
literal_lex!(rbrace_punctuation, "}", Tok::RBrace);
literal_lex!(semicolon_punctuation, ";", Tok::Semicolon);
literal_lex!(colon_punctuation, ":",Tok::Colon);
literal_lex!(rarrow_punctuation, "->", Tok::RightArrow);
literal_lex!(comma_punctuation, ",", Tok::Comma);

fn lex_punctuations(input: &[u8]) -> IResult<&[u8], Tok> {
    alt((
        lparen_punctuation,
        rparen_punctuation,
        lbrace_punctuation,
        rbrace_punctuation,
        semicolon_punctuation,
        rarrow_punctuation,
        colon_punctuation,
        comma_punctuation
    ))(input)
}

fn lex_integer(input: &[u8]) -> IResult<&[u8], Tok> {
    map(
        map_res(
            map_res(digit1, std::str::from_utf8),
            FromStr::from_str,
        ),
        Tok::Int,
    )(input)
}

fn lex_float(input: &[u8]) -> IResult<&[u8], Tok> {
    map(
        map_res(
            map_res(
                alt((
                    recognize(tuple((digit1, char('e'), digit1))),
                    recognize(tuple((digit1, char('.'), digit1, opt(pair(char('e'), digit1)))))
                )),
                std::str::from_utf8),
            FromStr::from_str,
        ),
        Tok::Float,
    )(input)
}

fn lex_ident_and_keyword(input: &[u8]) -> IResult<&[u8], Tok> {
    map_res(
        recognize(pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_"))))
        )),
        |word| {
            let word = std::str::from_utf8(word);
            word.map(|syntax| match syntax {
                "fn" => Tok::KwdFn,
                "true" => Tok::Bool(true),
                "false" => Tok::Bool(false),
                "return" => Tok::KwdRet,
                "val" => Tok::KwdVal,
                "var" => Tok::KwdVar,
                "import" => Tok::KwdImport,
                "if" => Tok::KwdIf,
                "else" => Tok::KwdElse,
                "elif" => Tok::KwdElif,
                _ => Tok::Ident(syntax.to_string())
            })
        }
    )(input)
}

fn lex_token(input: &[u8]) -> IResult<&[u8], Tok> {
    alt((
        lex_punctuations,
        lex_operator,
        lex_ident_and_keyword,
        lex_float,
        lex_integer,
    ))(input)
}

fn lex_tokens(input: &[u8]) -> IResult<&[u8], Vec<Tok>> {
    many0(delimited(multispace0, lex_token, multispace0))(input)
}

pub struct Lexer;
impl Lexer{
    pub fn lex_tokens(input: &[u8]) -> Vec<Tok> {
        let (i1, tok) = lex_tokens(input).unwrap();
        assert_eq!(i1.len(), 0);
        tok
    }
}
