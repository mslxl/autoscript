use aalang::cmd::{CmdOption, CmdParser, CmdReceiveType, CmdReceiveValue};
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
    let parser = cmd_parser();
    let mut args = env::args();
    args.next();
    match parser.parse(args) {
        Err(ref s)=>{
            print!("error: {}\n\n{}",s, parser.help_str());
            exit(-1)
        }
        _ => (),
    };

    if parser.is_empty() || parser.get_bool("help").unwrap_or(true) {
        print!("{}", parser.help_str());
        exit(0)
    }

    let files = parser.get_suffix();

    println!("{:?}",files)
}