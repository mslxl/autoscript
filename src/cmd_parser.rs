use std::env::Args;
pub struct CmdOption {
    pub print_ast: bool,
    pub print_help: bool,
    pub file_name: Option<String>,
}

impl CmdOption {
    fn new() -> CmdOption {
        CmdOption {
            print_ast: false,
            print_help: false,
            file_name: None,
        }
    }
    fn parse_next<'a>(
        args: &mut Args,
        template: &'a mut CmdOption,
    ) -> Result<&'a CmdOption, String> {
        let arg = args.next();
        if arg == Option::None {
            return Result::Ok(template);
        }
        let c: &str = &arg.unwrap();
        match c {
            "--print-ast" => {
                template.print_ast = true;
                CmdOption::parse_next(args, template)
            }
            "--help" | "-h" => {
                template.print_help = true;
                CmdOption::parse_next(args, template)
            }
            _ => {
                if c.starts_with("-") {
                    template.print_help = true;
                    Result::Err(format!("Illegal option \"{}\"", c))
                } else {
                    template.file_name = Some(String::from(c));
                    CmdOption::parse_next(args, template)
                }
            }
        }
    }
    pub fn parse(args: &mut Args) -> Result<CmdOption, String> {
        let mut opt = CmdOption::new();
        args.next();
        let res = CmdOption::parse_next(args, &mut opt);
        match res {
            Err(s) => Result::Err(s),
            Ok(_) => Result::Ok(opt),
        }
    }
}
