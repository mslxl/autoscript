use crate::{Lexer, Parser};

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
}