mod vm;
mod frontend;
use std::io;
use std::rc::Rc;
use crate::frontend::codegen::CodeGen;
use crate::frontend::lexer::Lexer;
use crate::frontend::parser::Parser;
use crate::frontend::tok::Tokens;
use crate::vm::interp::{AutoScriptLoader, AutoScriptModule, AutoScriptVM, FunctionPrototype};

fn main() {
    let mut buff = String::new();
    io::stdin().read_line(&mut buff).expect("Read content failed!");

    let (_, tokens) = Lexer::lex_tokens(buff.as_bytes()).unwrap();
    let tokens = Tokens::new(&tokens);
    let (_, expr) = Parser::parse(tokens).unwrap();
    let instr = Rc::new(CodeGen::new().translate_expr(*expr).instr);
    print!("{}", instr);
    let mut module = AutoScriptModule::default();

    module.insert_function_prototype("main", FunctionPrototype{
        name: String::from("main"),
        local_var_size: 0,
        code: instr
    });

    let mut loader = AutoScriptLoader::new();
    loader.put_module("internal", module);
    let mut vm = AutoScriptVM::new(loader);
    vm.start("internal");

}
