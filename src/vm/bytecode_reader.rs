use std::rc::Rc;
use crate::vm::instr::{Instr, Instructions};

pub trait AutoScriptInstrReader{
    fn read_instr(&mut self) -> Instr;
    fn set_pc(&mut self, pc:usize);
    fn pc(&self) -> usize;
}

pub struct InstrReader{
    instr: Rc<Instructions>,
    pc: usize,
}

impl AutoScriptInstrReader for InstrReader{
    fn read_instr(&mut self) -> Instr {
        let instr = self.instr.get_instr(self.pc);
        self.pc+=1;
        instr.unwrap()
    }
    fn set_pc(&mut self, pc: usize) {
        self.pc += pc;
    }



    fn pc(&self) -> usize {
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
    pub fn is_unfinished(&self) -> bool {
        return self.pc < self.instr.len()
    }

    pub fn reset(&mut self, instr: Rc<Instructions>, pc:usize){
        self.instr = instr;
        self.pc = pc;
    }
}