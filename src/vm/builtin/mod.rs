use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;

use crate::frontend::ast::basic::TypeInfo;
use crate::frontend::ast::func::{FunctionBasicInfo, FunctionMatcher, FunctionOrigin};
use crate::frontend::module_man::ProgramModuleDecl;
use crate::vm::builtin::builtin_func::{FnAssert, FnPrint};
use crate::vm::slot::Slot;
use crate::vm::thread::Frame;
use crate::vm::vm::AutoScriptFunctionEvaluator;

pub mod builtin_class;
pub mod builtin_func;

trait AsEvaluator  {
    fn as_evaluator(self)-> Box<dyn AutoScriptFunctionEvaluator>;
}


// impl AsEvaluator for Box<dyn FunctionRustBinding> {
//     fn as_evaluator(self) -> Box<dyn AutoScriptFunctionEvaluator> {
//         (*self).as_evaluator()
//     }
// }

impl <T> AsEvaluator for  Box<T> where T: FunctionRustBinding +  Sized + 'static {
    fn as_evaluator(self) -> Box<dyn AutoScriptFunctionEvaluator> {
        self
    }
}

impl <T> AutoScriptFunctionEvaluator for T where T:FunctionRustBinding {
    fn exec(&self, frame: &mut Frame) {
        let mut return_value = None;
        self.execute(frame, &mut return_value);
        let thread = unsafe { frame.thread.as_mut() }.unwrap();
        thread.frame_stack.pop();

        if let Some(slot) = return_value {
            thread.current_frame_mut().operand_stack.push(slot);
        }
    }
}

pub trait FunctionRustBinding: Debug  + AutoScriptFunctionEvaluator {
    fn get_name(&self) -> &'static str;
    fn get_args(&self) -> &'static [(&'static str, TypeInfo)];
    fn get_ret_type(&self) -> TypeInfo;

    fn execute(&self, frame: &mut Frame, ret: &mut Option<Slot>);
}

#[derive(Clone)]
pub struct ProgramVmFnElement {
    pub header: FunctionBasicInfo,
    pub block: Rc<dyn AutoScriptFunctionEvaluator>,
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

fn register_fn<T>(fn_map: &mut HashMap<String, Vec<ProgramVmFnElement>>, fn_code: Box<T>) where T:Sized + FunctionRustBinding + 'static {
    let name = (&fn_code.get_name()).to_string();
    let fn_prototype = ProgramVmFnElement {
        header: FunctionBasicInfo {
            name: fn_code.get_name().to_string(),
            module: Some(String::from("prelude")),
            param: Some(fn_code.get_args().into_iter().map(|(fst, snd)| (fst.to_string(), snd.clone())).collect()),
            ret: None,
            origin: FunctionOrigin::FFI
        },
        block: Rc::from(fn_code.as_evaluator()),
    };
    fn_map.insert(name, vec![fn_prototype]);
}

