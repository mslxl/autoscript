use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;
use std::sync::Arc;

use crate::vm::builtin::FunctionRustBinding;
use crate::vm::instr::Instructions;
use crate::vm::mem::Mem;
use crate::vm::slot::Slot;
use crate::vm::thread::{Frame, Thread};
use crate::VmArgs;

use super::const_pool::ConstantPool;

pub type FnSignature = String;
#[derive(Debug)]
pub struct AutoScriptPrototype {
    // temporary implementations
    functions: HashMap<FnSignature, Rc<AutoScriptFunction>>,
    constant_pool: ConstantPool
}

impl AutoScriptPrototype {
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
            constant_pool: vec![].into()
        }
    }
    pub fn insert_function_prototype(&mut self, signature: FnSignature, prototype: AutoScriptFunction) {
        self.functions.insert(signature, Rc::new(prototype));
    }
    pub fn get_function_prototype(&self, signature: &str) -> Option<Rc<AutoScriptFunction>> {
        self.functions.get(signature).map(Rc::clone)
    }

    pub fn replace_constant_pool(&mut self, pool: ConstantPool) -> ConstantPool {
        std::mem::replace(&mut self.constant_pool, pool)
    }
    pub fn get_constant(&self, idx:usize) -> Option<Slot> {
        self.constant_pool.get(idx)
    }
}

pub trait AutoScriptFunctionEvaluator : Debug{
    fn exec(&self, frame: &mut Frame);
}
#[derive(Debug)]
pub struct AutoScriptFunction {
    pub name: String,
    pub signature: String,
    pub local_var_size: usize,
    pub arg_num: usize,
    pub code: Rc<dyn AutoScriptFunctionEvaluator>,
}

impl AutoScriptFunctionEvaluator for AutoScriptFunction {
    #[inline]
    fn exec(&self, frame: &mut Frame) {
        self.code.exec(frame)
    }
}


pub struct AutoScriptVM {
    pub prototypes: AutoScriptPrototype,
    pub main_thread: Thread,
    pub mem: Arc<Mem>,
    pub args: VmArgs,
}

impl AutoScriptVM {
    pub fn new(prototypes: AutoScriptPrototype, args: VmArgs) -> Self {
        let mut interp = unsafe {
            Self {
                prototypes,
                main_thread: Thread::new_dangle(),
                mem: Arc::new(Mem::new()),
                args,
            }
        };
        let interp_ptr: *mut AutoScriptVM = (&mut interp) as *mut AutoScriptVM;
        interp.main_thread.switch_interp(interp_ptr);
        interp
    }

    pub fn start(&mut self, function_signature: &str) {
        self.main_thread.start(function_signature)
    }

    fn new_thread(&mut self) -> Thread {
        Thread::new(self)
    }
}