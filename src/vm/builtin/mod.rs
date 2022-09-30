use std::collections::HashMap;
use crate::frontend::ast::{FunctionHeader, FunctionOrigin, ProgramVmFnElement};
use crate::frontend::module_man::ProgramSrcModule;
use crate::vm::builtin::builtin_func::{AutoScriptVmFnCode, FnAssert, FnPrint};

pub mod builtin_number;
pub mod builtin_func;

pub struct VMBuiltinRegister;

impl VMBuiltinRegister{
    pub fn register_prelude(map: &mut HashMap<String, ProgramSrcModule>) {
        let mut module = ProgramSrcModule::default();
        register_fn(&mut module.vm_function, Box::new(FnAssert));
        register_fn(&mut module.vm_function, Box::new(FnPrint));
        map.insert(String::from("prelude"), module);
    }
}




fn register_fn(fn_map: &mut HashMap<String, Vec<ProgramVmFnElement>>, fn_code: Box<dyn AutoScriptVmFnCode>) {
    let name = (&fn_code.name()).to_string();
    let fn_prototype = ProgramVmFnElement {
        header: FunctionHeader {
            name: fn_code.name().to_string(),
            module: Some(String::from("prelude")),
            param: Some(fn_code.args().into_iter().map(|(fst, snd)| (fst.to_string(), snd.clone())).collect()),
            ret: None,
            origin: FunctionOrigin::VM
        },
        block: fn_code,
    };
    fn_map.insert(name, vec![fn_prototype]);
}

