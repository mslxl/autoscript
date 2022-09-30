use std::fmt::Debug;
use crate::frontend::ast::TypeInfo;
use crate::vm::slot::Slot;
use crate::vm::thread::Frame;

pub trait AutoScriptVmFnCode: Debug + VmFnCodeClone {
    fn name(&self) -> &'static str;
    fn args(&self) -> &'static [(&'static str, TypeInfo)];
    fn ret_type(&self) -> TypeInfo;
    fn execute(&self, args: &[Slot], frame: &mut Frame) -> Option<Slot>;
}

pub trait VmFnCodeClone {
    fn clone_box(&self) -> Box<dyn AutoScriptVmFnCode>;
}

impl<T> VmFnCodeClone for T where T: 'static + AutoScriptVmFnCode + Clone,
{
    fn clone_box(&self) -> Box<dyn AutoScriptVmFnCode> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn AutoScriptVmFnCode> {
    fn clone(&self) -> Box<dyn AutoScriptVmFnCode> {
        self.clone_box()
    }
}

#[derive(Debug, Clone)]
pub struct FnAssert;

impl AutoScriptVmFnCode for FnAssert {
    fn name(&self) -> &'static str {
        "assert"
    }

    fn args(&self) -> &'static [(&'static str, TypeInfo)] {
        &[("expr", TypeInfo::Bool)]
    }

    fn ret_type(&self) -> TypeInfo {
        TypeInfo::Unit
    }

    fn execute(&self, args: &[Slot], frame: &mut Frame) -> Option<Slot> {
        assert!(args.first().unwrap().get_bool());
        None
    }
}

#[derive(Debug, Clone)]
pub struct FnPrint;
impl AutoScriptVmFnCode for FnPrint{
    fn name(&self) -> &'static str {
        "print"
    }

    fn args(&self) -> &'static [(&'static str, TypeInfo)] {
        &[("msg", TypeInfo::Int)]
    }

    fn ret_type(&self) -> TypeInfo {
        TypeInfo::Unit
    }

    fn execute(&self, args: &[Slot], frame: &mut Frame) -> Option<Slot> {
        println!("{}", args.first().unwrap().to_string());
        None
    }
}