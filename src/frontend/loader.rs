use std::{env, fs};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use crate::frontend::ast::element::ProgramElement;
use crate::frontend::lexer::Lexer;
use crate::frontend::module_man::ProgramModuleDecl;
use crate::frontend::parser::Parser;
use crate::frontend::tok::Tokens;

pub struct ScriptFileLoader {
    load_path: Vec<PathBuf>,
    file_queue: Vec<PathBuf>,
    loaded_file_set: HashSet<PathBuf>,
    loaded_module: HashMap<String, Vec<ProgramElement>>,
}

impl ScriptFileLoader {
    pub fn new() -> Self {
        Self {
            load_path: vec![env::current_dir().unwrap()],
            file_queue: Vec::new(),
            loaded_file_set: Default::default(),
            loaded_module: HashMap::new(),
        }
    }

    fn add_module(&mut self, name: &str, parent: Option<&PathBuf>) -> Result<(), ()> {
        if let Some(parent_file) = parent {
            let file_to_load = parent_file.parent().unwrap().join(name).with_extension("aa");
            self.file_queue.push(file_to_load);
            return Ok(())
        }

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
            if !self.loaded_file_set.contains(&file) {
                self.add_file(&file)?;
            }
        }
        Ok(())
    }

    pub fn add_file(&mut self, path: &PathBuf) -> Result<(), ()> {
        let file = path.canonicalize().unwrap();
        let name = file.file_stem().unwrap().to_str().unwrap();

        let code = fs::read_to_string(&file).unwrap();
        self.loaded_file_set.insert(file.clone());

        let token = Lexer::lex_tokens(code.as_bytes());
        let programs = Parser::parse(Tokens::new(&token), name)
            .into_iter()
            .filter(|e| {
                match &e {
                    ProgramElement::Import(module_name) => {
                        self.add_module(module_name, Some(&file)).unwrap();
                        false
                    }
                    _ => true
                }
            }).collect();

        self.loaded_module.insert(name.to_string(), programs);

        Ok(())
    }
    pub fn unwrap(mut self) -> HashMap<String, ProgramModuleDecl> {
        let _ = &self.load_from_queue().unwrap();

        let mut map: HashMap<String, ProgramModuleDecl> = HashMap::new();
        for (module_name, element_vec) in self.loaded_module {
            let mut functions = HashMap::new();
            for element in element_vec {
                match element {
                    ProgramElement::Function(f) => {
                        if !functions.contains_key(&f.header.name) {
                            functions.insert(f.header.name.clone(), Vec::new());
                        }
                        functions.get_mut(&f.header.name).unwrap().push(f);
                    }
                    _ => todo!("以后会有其他的元素, e.g. {:?}", element)
                }
            }

            let module = ProgramModuleDecl {
                function: functions,
                vm_function: Default::default()
            };
            map.insert(module_name, module);
        }
        map
    }
}