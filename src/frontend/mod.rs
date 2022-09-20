use crate::{Lexer, Parser};
use crate::frontend::codegen::CodeGen;

pub mod ast;
pub mod codegen;
pub mod lexer;
pub mod parser;



#[test] fn parse_test(){
    fn test_expr(code:&str, is_err:bool){
        let lexer = Lexer::new(code);
        let mut parser = Parser::new(lexer);
        match parser.parse() {
            Ok(expr) => {
                println!("{}\t=> {:?}", code , expr);
                assert!(!is_err)
            },
            Err(e)=> {
                eprintln!("{}\t=>\n{}" ,code,e);
                assert!(is_err)
            }
        }
    }

    test_expr("1+1", false);
    test_expr("1++1", false);
    test_expr("1+-1", false);
    test_expr("1 + 3 * 2", false);
    test_expr("1 + 3 * -2" , false);
    test_expr("1 ** 2", true);
    test_expr("1--1", false);
    test_expr("1---1", false);
    test_expr("1+3*(2--1)", false);
    test_expr("1+-(2*3)", false);

    test_expr("1 < 2" ,false);
    test_expr("1 <= 2" ,false);
    test_expr("1 <= 2 == 3 > (2 - 1 * (4+2))" ,false);
    test_expr("114 % 3", false);
    test_expr("(114514)", false);
    test_expr("(1145) % 14", false);

    test_expr("3.14159 * 2",false);
    test_expr("11.1414 % 3 + (39 - 10) * 1.0 / 20", false);
}

#[test] fn basic_trans(){
    fn trans(code: &str){
        let lexer = Lexer::new(code);
        let mut parser = Parser::new(lexer);
        let mut generator = CodeGen::new();

        let expr = parser.parse().unwrap();
        let instr = generator.translate_expr(expr);
        println!(">>> {} >>>", code);
        print!("{}", instr.instr);
        println!("<<< {} <<<", code)
    }
    trans("1+5*3");
    trans("1+ -5");
    trans("114514 * (1919 + 810) % 400 - 10000");
    trans("11.1414 % 3 + (39 - 10) * 1.0 / 20");
}