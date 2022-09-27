use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use crate::vm::instr::Instructions;
use crate::vm::mem::Mem;
use crate::vm::thread::Thread;

pub type AutoScriptModuleMan = HashMap<String, AutoScriptModule>;

pub type FnSignature = String;
#[derive(Debug)]
pub struct AutoScriptModule {
    // temporary implementations
    name: String,
    functions: HashMap<FnSignature, Rc<FunctionPrototype>>,
}

impl AutoScriptModule {
    pub fn new(name: String) -> Self {
        Self {
            name,
            functions: HashMap::new(),
        }
    }
    pub fn insert_function_prototype(&mut self, signature: FnSignature, prototype: FunctionPrototype) {
        self.functions.insert(signature, Rc::new(prototype));
    }
    pub fn get_function_prototype(&self, signature: &str) -> Option<Rc<FunctionPrototype>> {
        self.functions.get(signature).map(Rc::clone)
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

#[derive(Debug)]
pub struct AutoScriptVM {
    pub module_man: AutoScriptModuleMan,
    main_thread: Thread,
    pub mem: Arc<Mem>,
}

impl AutoScriptVM {
    pub fn new(modules: AutoScriptModuleMan) -> Self {
        let mut interp = unsafe {
            Self {
                module_man: modules,
                main_thread: Thread::new_dangle(),
                mem: Arc::new(Mem::new()),
            }
        };
        let interp_ptr: *mut AutoScriptVM = (&mut interp) as *mut AutoScriptVM;
        interp.main_thread.switch_interp(interp_ptr);
        interp
    }

    pub fn start(&mut self, start_module: &str) {
        self.main_thread.start(start_module, "V@main(V")
    }

    fn new_thread(&mut self) -> Thread {
        Thread::new(self)
    }
}