#[macro_use]
extern crate lalrpop_util;

pub mod cmd;
pub mod ast;
lalrpop_mod!(pub parser);