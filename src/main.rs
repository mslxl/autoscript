use lalrpop_util::lalrpop_mod;
use std::fs;

lalrpop_mod!(pub aalang);
use crate::aalang::AALangParser;

fn main() {
    print!("> ");
    let content = fs::read_to_string("test_sample.aa").expect("fail to read test_sample.aa file");
    let code = content.trim();

    let result = AALangParser::new().parse(code);
    match result {
        Ok(ast) => {
            println!("AST result:");
            println!("{:?}", ast)
        },
        Err(e)=> println!("{}",e),
    }
}
