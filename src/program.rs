use std::collections::{HashMap, HashSet};

use lalrpop_util::{ParseError, lexer::Token};

use crate::{ast::{FuncDecl, VarDeclExpr, TopLevelScopeDecl}, parser::ProgramParser};


#[derive(Debug)]
pub struct Program {
    loaded_module: HashSet<String>,
    decl_func: HashMap<String,FuncDecl>,
    decl_var: HashMap<String,VarDeclExpr>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            loaded_module: HashSet::new(),
            decl_func: HashMap::new(),
            decl_var: HashMap::new(),
            //load_content: load_content_func,
        }
    }
    pub fn load<'a>(
        &'a mut self,
        code: &'a str,
    ) -> Result<(), ParseError<usize, Token<'_>, &'static str>> {
        let parser = ProgramParser::new();
        let ast = parser.parse(code)?;
        for i in ast {
            match i {
                TopLevelScopeDecl::FuncDecl(f)=>{
                    self.decl_func.insert(f.name.clone(), *f);
                }
                TopLevelScopeDecl::Import(s)=>{
                    todo!()
                }
                TopLevelScopeDecl::VarDecl(v)=>{
                    self.decl_var.insert(v.name.clone(),*v);

                }
            }
        }
        Ok(())
    }
}
