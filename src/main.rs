mod error;
mod frontend;


use std::io;
use std::io::Write;

use crate::frontend::lexer::Lexer;
use crate::frontend::parser::Parser;

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

