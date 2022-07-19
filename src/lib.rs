#[macro_use]
extern crate lalrpop_util;

pub mod interp;

pub mod cmd;
pub mod ast;
pub mod errors;
pub mod analysis;
pub mod program;
lalrpop_mod!(pub parser);