use std::fmt::Debug;

use crate::frontend::ast::basic::TypeInfo;
use crate::vm::builtin::FunctionRustBinding;
use crate::vm::slot::Slot;
use crate::vm::thread::Frame;
use crate::vm::vm::AutoScriptFunctionEvaluator;

#[derive(Debug, Clone)]
pub(crate) struct FnPrint;


impl FunctionRustBinding for FnPrint{
    fn get_name(&self) -> &'static str {
        "print"
    }

    fn get_args(&self) -> &'static [(&'static str, TypeInfo)] {
        &[("msg", TypeInfo::Any)]
    }

    fn get_ret_type(&self) -> TypeInfo {
        TypeInfo::Unit
    }

    fn execute(&self, frame: &mut Frame, _: &mut Option<Slot>) {
        let value = frame.local_vars.get(0);
        println!("{}", value.to_string());
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

    fn execute(&self, frame: &mut Frame, _: &mut Option<Slot>) {
        assert!(frame.local_vars.get(0).get_bool())
    }
}