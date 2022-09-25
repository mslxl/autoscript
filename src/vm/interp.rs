use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use crate::AutoScriptLoader;
use crate::frontend::ast::{Program, TypeInfo};
use crate::vm::instr::Instructions;
use crate::vm::mem::Mem;
use crate::vm::thread::Thread;

pub type AutoScriptModuleMan = HashMap<String, AutoScriptModule>;

#[derive(Debug)]
pub struct AutoScriptModule{
    // temporary implementations
    name: String,
    functions: HashMap<String, FunctionPrototype>
}

impl AutoScriptModule{
    pub fn insert_function_prototype(&mut self,name:&str, prototype: FunctionPrototype){
        self.functions.insert(name.to_owned(), prototype);
    }
    pub fn get_function_prototype(&self, name:&str) -> Option<&FunctionPrototype>{
        self.functions.get(name)
    }
}

impl AutoScriptModule{
    fn new(name:String) -> Self{
        Self{
            name,
            functions: HashMap::new()
        }
    }
}


#[derive(Debug)]
pub struct FunctionPrototype{
    pub name: String,
    pub local_var_size: usize,
    pub code: Rc<Instructions>,
    pub ret: TypeInfo
}
#[derive(Debug)]
pub struct AutoScriptVM {
    pub module_man: AutoScriptModuleMan,
    main_thread: Thread,
    pub mem : Arc<Mem>
}

impl AutoScriptVM {
    pub fn new(modules: AutoScriptModuleMan) -> Self {
        let mut interp = unsafe{
            Self{
                module_man: modules,
                main_thread: Thread::new_dangle(),
                mem: Arc::new(Mem::new())
            }
        };
        let interp_ptr: *mut AutoScriptVM = (&mut interp) as *mut AutoScriptVM;
        interp.main_thread.switch_interp(interp_ptr);
        interp
    }

    pub fn start(&mut self, start_module: &str){
        self.main_thread.interpret(start_module, "main")
    }

    fn new_thread(&mut self) -> Thread{
        Thread::new(self)
    }
}