extern crate core;

mod vm;
mod frontend;

use std::{env, fs, io};
use std::ffi::OsString;
use std::rc::Rc;
use crate::frontend::codegen::CodeGen;
use crate::frontend::lexer::Lexer;
use crate::frontend::parser::Parser;
use crate::frontend::tok::Tokens;
use crate::vm::interp::{AutoScriptLoader, AutoScriptModule, AutoScriptVM, FunctionPrototype};

fn main() {
    let args:Vec<OsString> = env::args_os().collect();
    assert_ne!(args.len(), 1);
    let buff = fs::read_to_string(args.get(1).unwrap()).unwrap();

    let tokens = Lexer::lex_tokens(buff.as_bytes());
    let tokens = Tokens::new(&tokens);
    let expr = Parser::parse(tokens);
    let func =CodeGen::new().translate_program(expr);
    let mut module = AutoScriptModule::default();
    println!("{:?}", func);
    print!("{}", func.code);
    module.insert_function_prototype(&func.name.clone(), func);

    let mut loader = AutoScriptLoader::new();
    loader.put_module("internal", module);
    let mut vm = AutoScriptVM::new(loader);
    vm.start("internal");

}
