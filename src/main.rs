use std::{env, fs};

use ::aalang::cmd_parser::CmdOption;
use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub aalang);
use crate::aalang::AALangParser;

fn print_help() {
    println!(
        "Usage: {} [OPTION]... [FILE]",
        env::current_exe()
            .unwrap()
            .as_path()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
    );
    println!("Interpreter of aalang");
    println!("");
    println!("Mandatory arguments to long options are mandatory for short options too.");
    println!("      --print-ast\t print Abstract Syntax Tree, but don't exec code");
    println!("  -h, --help\t\tshow help( you are in here)");
}

fn launch(opt: &CmdOption) {
    if opt.print_help {
        print_help();
        return;
    }
    if opt.file_name == None {
        println!("fatal: no input file\n");
        print_help();
        return;
    }
    let file_name:&str = opt.file_name.as_ref().unwrap();

    let parser = AALangParser::new();


    let code = fs::read_to_string(file_name).expect("Fail to read source code");
    let ast  = match parser.parse(&code){
        Err(e) => panic!("{}", e),
        Ok(ast) => ast
    };

    if opt.print_ast {
        println!("{:#?}", ast);
        return
    }

    panic!("{}","TODO")


}

fn main() {
    let mut args = env::args();
    let res = CmdOption::parse(&mut args);
    match res {
        Result::Err(msg) => {
            println!("{}\n", msg);
            print_help();
        }
        Result::Ok(opt) => launch(&opt),
    }
}
