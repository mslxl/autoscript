use std::ptr::null_mut;
use std::rc::Rc;
use crate::vm::bytecode_reader::{AutoScriptInstrReader, InstrReader};
use crate::vm::interp::{AutoScriptVM, FunctionPrototype};
use crate::vm::slot::Slot;


#[derive(Debug)]
pub struct Thread {
    name: String,
    pc: usize,
    pub frame_stack: Vec<Frame>,
    pub vm: *mut AutoScriptVM,
}

unsafe impl Send for Thread {}

impl Thread {
    pub fn new(interp: &mut AutoScriptVM) -> Self {
        let interp_ptr = interp as *mut AutoScriptVM;
        Self {
            name: String::from("unnamed_thread"),
            pc: 0,
            frame_stack: Vec::new(),
            vm: interp_ptr,
        }
    }
    pub fn rename(&mut self, new_name: String) {
        self.name = new_name
    }

    pub unsafe fn new_dangle() -> Self {
        Self {
            name: String::from("unnamed_thread"),
            pc: 0,
            frame_stack: Vec::new(),
            vm: null_mut(),
        }
    }

    pub fn switch_interp(&mut self, interp: *mut AutoScriptVM) {
        self.vm = interp;
    }
    pub fn pop_frame(&mut self) -> Option<Frame> {
        self.frame_stack.pop()
    }

    pub fn push_frame(&mut self, frame: Frame) -> &Frame {
        self.frame_stack.push(frame);
        self.frame_stack.last().unwrap()
    }
    pub fn push_new_frame(&mut self, slot_size: usize, instr: Rc<FunctionPrototype>) -> &mut Frame {
        let frame = Frame::new(slot_size, instr, self);
        self.push_frame(frame);
        self.frame_stack.last_mut().unwrap()
    }

    pub fn current_frame_mut(&mut self) -> &mut Frame {
        self.frame_stack.last_mut().unwrap()
    }
    pub fn current_frame(&self) -> &Frame {
        self.frame_stack.last().unwrap()
    }

    fn loop_interpret(&mut self) {
        let mut instr_reader = InstrReader::new(Rc::clone(&self.current_frame_mut().function.code));
        loop {
            let frame = self.current_frame();
            let pc = frame.next_pc;
            self.pc = pc;

            let mut frame = self.current_frame_mut();
            // decode
            instr_reader.reset(Rc::clone(&frame.function.code), pc);
            let instr = instr_reader.read_instr();
            frame.next_pc = instr_reader.pc();
            instr.execute(frame);


            if self.frame_stack.is_empty() {
                break;
            }
        }
    }


    pub fn start(&mut self, module_name: &str, function_signature: &str) {
        let vm: &mut AutoScriptVM = unsafe { &mut *self.vm };
        let module = vm.module_man.get(module_name).unwrap();
        let function = module.get_function_prototype(function_signature).unwrap();
        self.push_new_frame(function.local_var_size, Rc::clone(&function));
        self.loop_interpret();
    }
    pub fn pc(&self) -> usize {
        self.pc
    }
}

#[derive(Debug)]
pub struct Frame {
    pub local_vars: LocalVars,
    pub operand_stack: Vec<Slot>,
    pub next_pc: usize,
    pub function: Rc<FunctionPrototype>,
    pub thread: *mut Thread,
}

impl Frame {
    fn new(size: usize, instr: Rc<FunctionPrototype>, ptr: &mut Thread) -> Self {
        Self {
            local_vars: LocalVars::with_cap(size),
            operand_stack: Vec::new(),
            next_pc: 0,
            function: instr,
            thread: ptr as *mut Thread,
        }
    }
}

#[derive(Debug)]
pub struct LocalVars(Vec<Slot>);

impl LocalVars {
    fn with_cap(size: usize) -> Self {
        Self(vec![Slot::Unit; size])
    }

    pub fn set(&mut self, index: usize, slot: Slot) {
        self.0[index] = slot;
    }
    pub fn get(&self, index: usize) -> &Slot {
        unsafe {
            self.0.get_unchecked(index)
        }
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
}
