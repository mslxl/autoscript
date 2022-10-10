use crate::frontend::ast::basic::{StmtBlock, TypeInfo};
use crate::frontend::ast::func::{FunctionBasicInfo, FunctionMatcher};

#[derive(Debug, PartialEq)]
pub enum ProgramElement {
    Import(String),
    Function(AstProgramFunctionImplElement),
    Class(ProgramClassElement)
}

impl ProgramElement {
    pub fn set_module(self, module_name: String) -> Self {
        match self {
            ProgramElement::Import(_) => self,
            ProgramElement::Function(mut e) => {
                e.header.module = Some(module_name);
                ProgramElement::Function(e)
            }

            ProgramElement::Class(mut e) => {
                e.module = module_name;
                ProgramElement::Class(e)
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
/// Parser will save source info to this struct
/// This struct wouldn't exists long, instr will be generate in
pub struct AstProgramFunctionImplElement {
    pub header: FunctionBasicInfo,
    pub block: StmtBlock,
}

impl FunctionMatcher for AstProgramFunctionImplElement {
    fn is_executable_by(&self, name: &str, param: Option<&Vec<TypeInfo>>) -> bool {
        self.header.is_executable_by(name, param)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProgramClassElement {
    pub name: String,
    pub module: String
}