use std::fmt::Debug;

use crate::frontend::ast::basic::TypeInfo;
use crate::vm::builtin::FunctionRustBinding;
use crate::vm::slot::Slot;
use crate::vm::thread::Frame;

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

    fn execute(&self, args: &[Slot], _: &mut Frame) -> Option<Slot> {
        println!("{}", args.first().unwrap().to_string());
        None
    }
}