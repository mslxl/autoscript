use std::rc::Rc;

use crate::vm::instr::{Instr, Instructions};

pub trait AutoScriptInstrReader{
    fn read_instr(&mut self) -> Instr;
    fn set_pc(&mut self, pc:i32);
    fn pc(&self) -> i32;
}

pub struct InstrReader{
    instr: Rc<Instructions>,
    pc: i32,
}

impl AutoScriptInstrReader for InstrReader{
    fn read_instr(&mut self) -> Instr {
        let instr = self.instr.get_instr(self.pc);
        self.pc+=1;
        instr.unwrap()
    }
    fn set_pc(&mut self, pc: i32) {
        self.pc += pc;
    }

    fn pc(&self) -> i32 {
        self.pc
    }
}

impl InstrReader{
    pub fn new(data: Rc<Instructions>) -> Self {
        InstrReader{
            instr: data,
            pc: 0
        }
    }

    pub fn reset(&mut self, instr: Rc<Instructions>, pc:i32){
        self.instr = instr;
        self.pc = pc;
    }
}