use std::collections::HashMap;
use crate::frontend::ast::{FunctionCompare, FunctionHeader, ProgramSrcFnElement, ProgramVmFnElement, TypeInfo};

#[derive(Clone, Debug)]
pub struct ProgramSrcModule {
    pub function: HashMap<String, Vec<ProgramSrcFnElement>>,
    pub vm_function: HashMap<String, Vec<ProgramVmFnElement>>,
}

impl Default for ProgramSrcModule {
    fn default() -> Self {
        Self {
            function: Default::default(),
            vm_function: Default::default(),
        }
    }
}

impl ProgramSrcModule {
    pub fn search_function(&self, name: &str, param: Option<&Vec<TypeInfo>>) -> Option<&FunctionHeader> {
        if self.vm_function.contains_key(name) {
            let funcs = self.vm_function.get(name).unwrap();
            for item in funcs {
                if item.is_executable_by(name, param) {
                    return Some(&item.header)
                }
            }
            None
        } else if self.function.contains_key(name) {
            let funcs = self.function.get(name).unwrap();
            for item in funcs {
                if item.is_executable_by(name, param) {
                    return Some(&item.header);
                }
            }
            None
        } else {
            None
        }
    }
}
