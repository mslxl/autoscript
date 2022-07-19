use aalang::cmd::{CmdOption, CmdParser, CmdReceiveType, CmdReceiveValue};
use aalang::program::Program;
use std::env::Args;
use std::fs;
use std::{env, process::exit};

struct CmdConf {
    parser: CmdParser,

    is_empty: bool,

    print_ast: bool,
    human_readable: bool,
    // type_check: bool,
    help: bool,

    files: Vec<String>,
}

impl CmdConf {
    fn parser() -> CmdParser {
        let mut parser = CmdParser::new();
        parser.option(CmdOption::new(
            "print-ast".to_string(),
            None,
            "print abstract syntax tree, but don't eval it".to_string(),
            CmdReceiveType::None,
            Some(CmdReceiveValue::Bool(false)),
        ));
        parser.option(CmdOption::new(
            "human-readable".to_string(),
            None,
            "print data in format".to_string(),
            CmdReceiveType::None,
            Some(CmdReceiveValue::Bool(false)),
        ));
        parser.option(CmdOption::new(
            "check".to_string(),
            None,
            "only check the types of each expr, but don't eval it".to_string(),
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

    fn from(args: &mut Args) -> Result<CmdConf, String> {
        let parser = CmdConf::parser();
        parser.parse(args)?;
        Ok(CmdConf {
            is_empty: parser.is_empty(),
            print_ast: parser.get_bool("print-ast").unwrap_or(false),
            // type_check: parser.get_bool("check").unwrap_or(false),
            help: parser.get_bool("help").unwrap_or(false),
            human_readable: parser.get_bool("human-readable").unwrap_or(false),
            files: parser.get_suffix(),
            parser,
        })
    }
}

fn main() {
    let mut args = env::args();
    args.next();
    let conf = CmdConf::from(&mut args);
    if let Err(ref s) = conf {
        print!("error: {}\n\n{}", s, CmdConf::parser().help_str());
        exit(-1)
    }
    let conf = conf.unwrap();

    if conf.is_empty || conf.help {
        print!("{}", conf.parser.help_str());
        exit(0)
    }
    if conf.files.is_empty() {
        print!("error: no input file\n\n{}", conf.parser.help_str());
        exit(-1)
    }
    let mut program = Program::new();
    for file in conf.files{
        let code = fs::read_to_string(&file)
            .expect(&format!("Unable to read file '{}'", &file));
        program.load(&code).unwrap();
    }

    if conf.print_ast {
        if conf.human_readable {
            println!("{:#?}", program)
        } else{
            println!("{:?}", program)
        }
    }

}
