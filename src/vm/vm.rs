use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use crate::vm::slot::Slot;
use crate::vm::builtin::FunctionRustBinding;
use crate::vm::instr::Instructions;
use crate::vm::mem::Mem;
use crate::vm::thread::Thread;
use crate::VmArgs;

use super::const_pool::ConstantPool;


pub type FnSignature = String;
pub struct AutoScriptPrototype {
    // temporary implementations
    functions: HashMap<FnSignature, Rc<FunctionPrototype>>,
    vm_functions: HashMap<FnSignature, Box<dyn FunctionRustBinding>>,
    constant_pool: ConstantPool
}

impl AutoScriptPrototype {
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
            vm_functions: HashMap::new(),
            constant_pool: vec![].into()
        }
    }
    pub fn insert_function_prototype(&mut self, signature: FnSignature, prototype: FunctionPrototype) {
        self.functions.insert(signature, Rc::new(prototype));
    }
    pub fn get_function_prototype(&self, signature: &str) -> Option<Rc<FunctionPrototype>> {
        self.functions.get(signature).map(Rc::clone)
    }

    pub fn insert_vm_function(&mut self, signature: FnSignature, vm_func: Box<dyn FunctionRustBinding>) {
        self.vm_functions.insert(signature, vm_func);
    }
    pub fn get_vm_function(&self, signature: &str) -> Option<&Box<dyn FunctionRustBinding>> {
        self.vm_functions.get(signature)
    }

    pub fn replace_constant_pool(&mut self, pool: ConstantPool) -> ConstantPool {
        std::mem::replace(&mut self.constant_pool, pool)
    }
    pub fn get_constant(&self, idx:usize) -> Option<Slot> {
        self.constant_pool.get(idx)
    }
}


#[derive(Debug)]
pub struct FunctionPrototype {
    pub name: String,
    pub signature: String,
    pub local_var_size: usize,
    pub arg_num: usize,
    pub code: Rc<Instructions>,
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