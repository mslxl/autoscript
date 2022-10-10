use std::collections::HashMap;
use std::fmt::Debug;

use crate::frontend::ast::basic::TypeInfo;
use crate::frontend::ast::func::{FunctionBasicInfo, FunctionMatcher, FunctionOrigin};
use crate::frontend::module_man::ProgramModuleDecl;
use crate::vm::builtin::builtin_func::FnPrint;
use crate::vm::slot::Slot;
use crate::vm::thread::Frame;

pub mod builtin_class;
pub mod builtin_func;

pub trait FunctionRustBinding: Debug + FunctionRustBindingDynClone {
    fn get_name(&self) -> &'static str;
    fn get_args(&self) -> &'static [(&'static str, TypeInfo)];
    fn get_ret_type(&self) -> TypeInfo;
    fn execute(&self, args: &[Slot], frame: &mut Frame) -> Option<Slot>;
}

pub trait FunctionRustBindingDynClone {
    fn clone_box(&self) -> Box<dyn FunctionRustBinding>;
}

impl<T> FunctionRustBindingDynClone for T where T: 'static + FunctionRustBinding + Clone,
{
    fn clone_box(&self) -> Box<dyn FunctionRustBinding> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn FunctionRustBinding> {
    fn clone(&self) -> Box<dyn FunctionRustBinding> {
        self.clone_box()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct FnAssert;

impl FunctionRustBinding for FnAssert {
    fn get_name(&self) -> &'static str {
        "assert"
    }

    fn get_args(&self) -> &'static [(&'static str, TypeInfo)] {
        &[("expr", TypeInfo::Bool)]
    }

    fn get_ret_type(&self) -> TypeInfo {
        TypeInfo::Unit
    }

    fn execute(&self, args: &[Slot], _: &mut Frame) -> Option<Slot> {
        assert!(args.first().unwrap().get_bool());
        None
    }
}


#[derive(Debug, Clone)]
pub struct ProgramVmFnElement {
    pub header: FunctionBasicInfo,
    pub block: Box<dyn FunctionRustBinding>,
}

impl FunctionMatcher for ProgramVmFnElement {
    fn is_executable_by(&self, name: &str, param: Option<&Vec<TypeInfo>>) -> bool {
        self.header.is_executable_by(name, param)
    }
}


pub struct VMBuiltinRegister;

impl VMBuiltinRegister{
    pub fn register_prelude(map: &mut HashMap<String, ProgramModuleDecl>) {
        let mut module = ProgramModuleDecl::default();
        register_fn(&mut module.vm_function, Box::new(FnAssert));
        register_fn(&mut module.vm_function, Box::new(FnPrint));
        map.insert(String::from("prelude"), module);
    }
}

fn register_fn(fn_map: &mut HashMap<String, Vec<ProgramVmFnElement>>, fn_code: Box<dyn FunctionRustBinding>) {
    let name = (&fn_code.get_name()).to_string();
    let fn_prototype = ProgramVmFnElement {
        header: FunctionBasicInfo {
            name: fn_code.get_name().to_string(),
            module: Some(String::from("prelude")),
            param: Some(fn_code.get_args().into_iter().map(|(fst, snd)| (fst.to_string(), snd.clone())).collect()),
            ret: None,
            origin: FunctionOrigin::VM
        },
        block: fn_code,
    };
    fn_map.insert(name, vec![fn_prototype]);
}

