use std::collections::HashMap;

use crate::frontend::ast::basic::TypeInfo;
use crate::frontend::ast::element::AstProgramFunctionImplElement;
use crate::frontend::ast::func::FunctionBasicInfo;
use crate::vm::builtin::ProgramVmFnElement;

#[derive(Clone, Debug)]
pub struct ProgramModuleDecl {
    pub function: HashMap<String, Vec<AstProgramFunctionImplElement>>,
    pub vm_function: HashMap<String, Vec<ProgramVmFnElement>>,
}

impl Default for ProgramModuleDecl {
    fn default() -> Self {
        Self {
            function: Default::default(),
            vm_function: Default::default(),
        }
    }
}

impl ProgramModuleDecl {
    pub fn search_function(&self, name: &str, param: Option<&Vec<TypeInfo>>) -> Option<&FunctionBasicInfo> {
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
