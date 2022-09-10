extern crate core;

mod error;
mod lexer;
mod parser;
mod ast;

use std::io;
use lexer::Lexer;
use lexer::Tok;
use parser::Parser;

fn main() {
    // let mut lexer = Lexer::new("111 + 223 * 3");
    let mut buff = String::new();
    io::stdin().read_line(&mut buff).expect("Read content failed!");
    let mut lexer = Lexer::new(buff.trim());
    let mut parser = Parser::new(lexer);
    match parser.parse() {
        Ok(expr) => println!("{:?}", expr),
        Err(e)=> eprintln!("{}" ,e)
    }
}
