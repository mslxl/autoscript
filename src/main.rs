extern crate core;

mod lexer;
mod parser;
mod ast;

use lexer::Lexer;
use lexer::Tok;
use parser::Parser;

fn main() {
    let mut lexer = Lexer::new("111 + 223 * 3");
    let mut parser = Parser::new(lexer);
    println!("{:?}", parser.parse());
}
