mod error;
mod vm;
mod frontend;


use std::io;
use std::io::Write;
use std::rc::Rc;
use crate::frontend::codegen::CodeGen;

use crate::frontend::lexer::Lexer;
use crate::frontend::parser::Parser;
use crate::vm::interp::{AutoScriptLoader, AutoScriptModule, AutoScriptVM, FunctionPrototype};

fn main() {
    // let mut lexer = Lexer::new("111 + 223 * 3");
    let mut buff = String::new();
    io::stdin().read_line(&mut buff).expect("Read content failed!");
    let lexer = Lexer::new(buff.trim());
    let mut parser = Parser::new(lexer);
    let expr = if let Ok(expr) = parser.parse() {
        expr
    }else{
        panic!();
    };
}

#[test]
fn test_eval(){
    fn run_expr(code: &str){
        let mut parser = Parser::new(Lexer::new(code.trim()));
        let expr_node = parser.parse().unwrap();
        let mut generator = CodeGen::new();
        let instr =  Rc::new(generator.translate_expr(expr_node).instr);

        let mut module = AutoScriptModule::default();
        let mut main_function = FunctionPrototype{
            name: String::from("main"),
            local_var_size: 0,
            code: instr
        };
        module.insert_function_prototype("main", main_function);

        let mut loader = AutoScriptLoader::new();
        loader.put_module("internal", module);
        let mut vm = AutoScriptVM::new(loader);

        vm.start("internal")
    }

    run_expr("1+1");
    run_expr("114514 * 2");
    run_expr("114514 * 2 - 7");

    run_expr("1 + ---1");
    run_expr("1+3*(2--1)");
    run_expr("114514 * (1919 + 810) % 400 - 10000")
}
