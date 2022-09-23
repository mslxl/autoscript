use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use crate::frontend::ast::TypeInfo;
use crate::vm::instr::Instructions;
use crate::vm::mem::Mem;
use crate::vm::thread::Thread;

#[derive(Debug)]
pub struct AutoScriptLoader {
    load_path:Vec<String>,
    modules: HashMap<String, AutoScriptModule>
}

impl AutoScriptLoader{
    pub fn new() -> Self{
        Self{
            load_path: Vec::new(),
            modules: HashMap::new()
        }
    }
    pub fn request(&mut self, module_name:&str) -> Option<&AutoScriptModule>{
        if self.modules.contains_key(module_name) {
            self.modules.get(module_name)
        }else{
            if let Some(module) = self.load(module_name) {
                self.modules.insert(module_name.to_owned(), module);
                self.modules.get(module_name)
            }else{
                None
            }
        }
    }
    pub fn put_module(&mut self, module_name:&str, module:AutoScriptModule){
        self.modules.insert(module_name.to_owned(), module);
    }

    fn load(&mut self, module_name:&str) -> Option<AutoScriptModule>{
        self.load_local(module_name)
    }

    fn load_local(&mut self, module_name:&str) -> Option<AutoScriptModule>{
        todo!()
    }
}

#[derive(Debug)]
pub struct AutoScriptModule{
    // temporary implementations
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

impl Default for AutoScriptModule{
    fn default() -> Self {
        Self{
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
    pub loader: AutoScriptLoader,
    main_thread: Thread,
    pub mem : Arc<Mem>
}

impl AutoScriptVM {
    pub fn new(loader: AutoScriptLoader) -> Self {
        let mut interp = unsafe{
            Self{
                loader,
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