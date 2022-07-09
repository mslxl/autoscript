use aalang::cmd::{CmdOption, CmdParser, CmdReceiveType, CmdReceiveValue};
use aalang::parser::ProgramParser;
use std::fs;
use std::{env, process::exit};

fn cmd_parser()->CmdParser {
    let mut parser = CmdParser::new();
    parser.option(CmdOption::new(
        "print-ast".to_string(),
        None,
        "print abstract syntax tree, but don't eval it".to_string(),
        CmdReceiveType::None,
        Some(CmdReceiveValue::Bool(false)),
    ));
    parser.option(CmdOption::new(
        "help".to_string(),
        Some("h".to_string()),
        "print help".to_string(),
        CmdReceiveType::None,
        Some(CmdReceiveValue::Bool(false)),
    ));
    parser
}
fn main() {
    let cmd = cmd_parser();
    let mut args = env::args();
    args.next();
    match cmd.parse(args) {
        Err(ref s)=>{
            print!("error: {}\n\n{}",s, cmd.help_str());
            exit(-1)
        }
        _ => (),
    };

    if cmd.is_empty() || cmd.get_bool("help").unwrap_or(true) {
        print!("{}", cmd.help_str());
        exit(0)
    }

    let files = cmd.get_suffix();
    if files.is_empty() {
        print!("error: no input file\n\n{}", cmd.help_str());
        exit(-1)
    }

    let parser = ProgramParser::new();
    let src  = fs::read_to_string(&files[0]).expect("Fail to read file");

    let ast = parser.parse(&src);
    if cmd.get_bool("print-ast").unwrap_or(false) {
        println!("AST:\n{:#?}", ast);
        exit(0)
    }
    






}