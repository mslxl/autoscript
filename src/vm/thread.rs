use std::cell::RefCell;
use std::fs::read;
use std::io::Write;
use std::ptr::{null, null_mut};
use std::rc::Rc;
use std::sync::Mutex;
use std::thread;
use crate::AutoScriptLoader;
use crate::vm::bytecode_reader::{AutoScriptInstrReader, InstrReader};
use crate::vm::interp::AutoScriptVM;
use crate::vm::obj::{Obj, ObjInner};
use crate::vm::slot::Slot;


#[derive(Debug)]
pub struct Thread{
    name:String,
    pc:usize,
    frame_stack: Vec<Frame>,
    interp: *mut AutoScriptVM,
}

unsafe impl Send for Thread{}

impl Thread{
    pub fn new(interp: &mut AutoScriptVM) ->Self{
        let interp_ptr = interp as *mut AutoScriptVM;
        Self{
            name: String::from("unnamed_thread"),
            pc: 0,
            frame_stack: Vec::new(),
            interp: interp_ptr
        }
    }
    pub fn rename(&mut self, new_name:String){
        self.name = new_name
    }

    pub unsafe fn  new_dangle() -> Self{
        Self{
            name: String::from("unnamed_thread"),
            pc: 0,
            frame_stack: Vec::new(),
            interp: null_mut()
        }
    }

    fn current_frame(&self) -> &Frame {
        self.frame_stack.last().unwrap()
    }

    pub fn switch_interp(&mut self, interp: *mut AutoScriptVM){
        self.interp = interp;
    }

    fn push_frame(&mut self, cap_size:usize) -> &Frame{
        let frame = Frame::with_cap(cap_size);
        self.frame_stack.push(frame);
        self.frame_stack.last().unwrap()
    }

    pub fn interpret(&mut self, module_name:&str, function_name:&str){
        let vm: &mut AutoScriptVM = unsafe {&mut *self.interp};
        let module = vm.loader.request(module_name).unwrap();
        let function = module.get_function_prototype(function_name).unwrap();
        self.push_frame(function.local_var_size);

        let mut reader = InstrReader::new(Rc::clone(&function.code));

        let mut frame = self.frame_stack.pop().unwrap();

        println!(">>>");
        while reader.is_unfinished() {
            let instr = reader.read_instr();
            instr.execute(&mut frame);
            println!("{:?}", frame.operand_stack);
        }
        println!("<<<");

    }
}
#[derive(Debug)]
pub struct Frame{
    pub local_vars: LocalVars,
    pub operand_stack: Vec<Slot>,
}

impl Frame{
    fn with_cap(size:usize) -> Self{
        Self{
            local_vars: LocalVars::with_cap(size),
            operand_stack: Vec::new()
        }
    }
}

#[derive(Debug)]
pub struct LocalVars(Vec<Slot>);

impl LocalVars{
    fn with_cap(size:usize) -> Self{
        unsafe{
            Self(vec![Slot::Null; size])
        }
    }
}