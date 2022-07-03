use lalrpop_util::lalrpop_mod;
use std::io::{self, Write};

use crate::main::SParser;

lalrpop_mod!(pub main);

fn main() {
    print!("> ");
    io::stdout().flush().expect("Fail to flush stdout");
    let mut expr = String::new();
    io::stdin().read_line(&mut expr).expect("Fail to read expr from stdin");
    let expr = expr.trim();

    let result = SParser::new().parse(expr);
    match result {
        Ok(ast) => {
            println!("AST result:");
            println!("{:#?}", ast)
        },
        Err(e)=> println!("{}",e),
    }

}
