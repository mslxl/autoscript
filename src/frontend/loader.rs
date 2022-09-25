use std::collections::{HashMap, HashSet};
use std::{env, fs};
use std::path::PathBuf;
use crate::frontend::ast::Program;
use crate::frontend::codegen::CodeGen;
use crate::frontend::lexer::Lexer;
use crate::frontend::parser::Parser;
use crate::frontend::tok::Tokens;

pub struct AutoScriptLoader {
    load_path: Vec<PathBuf>,
    file_queue: Vec<PathBuf>,
    loaded_file: HashSet<PathBuf>,
    generator: CodeGen,

    modules: HashMap<String, Vec<Program>>,
}

impl AutoScriptLoader {
    pub fn new() -> Self {
        Self {
            load_path: vec![env::current_dir().unwrap()],
            file_queue: Vec::new(),
            loaded_file: Default::default(),
            generator: CodeGen::new(),
            modules: HashMap::new(),
        }
    }

    fn add_module(&mut self, name: &str) -> Result<(), ()> {
        for path in &self.load_path {
            let file_name = path.join(name);
            if file_name.exists() {
                self.file_queue.push(file_name);
                return self.load_from_queue();
            }
        }
        Err(())
    }

    fn load_from_queue(&mut self) -> Result<(), ()> {
        while !self.file_queue.is_empty() {
            let file = self.file_queue.pop().unwrap();
            if !self.loaded_file.contains(&file) {
                self.add_file(file)?;
            }
        }
        Ok(())
    }

    pub fn add_file(&mut self, path: PathBuf) -> Result<(), ()> {
        let file = path.canonicalize().unwrap();
        let name = &file.file_stem().unwrap();

        let code = fs::read_to_string(&file).unwrap();
        self.loaded_file.insert(file.clone());

        let token = Lexer::lex_tokens(code.as_bytes());
        let programs = Parser::parse(Tokens::new(&token))
            .into_iter()
            .filter(|e| {
                match &e {
                    Program::Import(module_name) => {
                        self.add_module(module_name).unwrap();
                        false
                    }
                    _ => true
                }
            }).collect();

        self.modules.insert(name.to_str().unwrap().to_owned(), programs);

        Ok(())
    }
    pub fn unwrap(self) -> HashMap<String, Vec<Program>>{
        self.modules
    }
}