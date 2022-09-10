mod error;
mod lexer;
mod parser;
mod ast;


use std::io;
use std::io::Write;
use lexer::Lexer;
use lexer::Tok;
use parser::Parser;

fn main() {
    // let mut lexer = Lexer::new("111 + 223 * 3");
    let mut buff = String::new();
    print!("Type expr to be parsed: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buff).expect("Read content failed!");
    let lexer = Lexer::new(buff.trim());
    let mut parser = Parser::new(lexer);
    match parser.parse() {
        Ok(expr) => println!("{:?}", expr),
        Err(e)=> eprintln!("{}" ,e)
    }
}

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
    test_expr("1---1", true);
    test_expr("1+3*(2--1)", false);
    test_expr("1+-(2*3)", false);
}