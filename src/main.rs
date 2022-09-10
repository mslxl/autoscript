extern crate core;

mod error;
mod lexer;
mod parser;
mod ast;

use lexer::Lexer;
use lexer::Tok;
use parser::Parser;

fn main() {
    // let mut lexer = Lexer::new("111 + 223 * 3");
    let mut lexer = Lexer::new("111 + +");
    let mut parser = Parser::new(lexer);
    match parser.parse() {
        Ok(expr) => println!("{:?}", expr),
        Err(e)=> eprintln!("{}" ,e)
    }
}
