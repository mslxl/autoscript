use std::error::Error;
use std::str::FromStr;
use nom::branch::alt;
use nom::IResult;
use nom::combinator::{map, map_res, opt, recognize};
use nom::bytes::complete::tag;
use nom::bytes::streaming::take_while_m_n;
use nom::character::complete::{char, digit0, digit1, multispace0};
use nom::multi::{many0, many_m_n};
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
        lt_op
    ))(input)
}

literal_lex!(lparen_punctuation, "(", Tok::LParen);
literal_lex!(rparen_punctuation, ")", Tok::RParen);
fn lex_punctuations(input: &[u8]) -> IResult<&[u8], Tok> {
    alt((
        lparen_punctuation,
        rparen_punctuation
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

fn lex_token(input: &[u8]) -> IResult<&[u8], Tok> {
    alt((
        lex_operator,
        lex_punctuations,
        lex_float,
        lex_integer,
    ))(input)
}

fn lex_tokens(input: &[u8]) -> IResult<&[u8], Vec<Tok>> {
    many0(delimited(multispace0, lex_token, multispace0))(input)
}

pub struct Lexer;
impl Lexer{
    pub fn lex_tokens(input: &[u8]) -> IResult<&[u8], Vec<Tok>> {
        lex_tokens(input)
    }
}

mod tests{
    use super::*;

    #[test]
    fn test_lexer() {
        println!("{:?}", Lexer::lex_tokens("1 + 3.14 * 4132 ".as_bytes()).unwrap())
    }
}


