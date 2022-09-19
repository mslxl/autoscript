use std::fmt::{Display, Formatter};
use std::ops;
use crate::vm::obj::Obj;
use crate::vm::slot::Slot;
use crate::vm::thread::Frame;

#[derive(Clone, Debug)]
pub enum Instr {
    IPush(i32),
    IAdd,
    ISub,
    IMul,
    IDiv,
    INeg,
    IRem,

    Call(String),
    Goto(i32),
}

impl Instr{
    pub fn execute(&self, frame: &mut Frame){
        match self{
            Instr::IPush(value) => {
                let slot = Slot::Int(*value as i64);
                frame.operand_stack.push(slot);
            },
            Instr::IAdd => {
                let v2 = frame.operand_stack.pop().unwrap().get_int();
                let v1 = frame.operand_stack.pop().unwrap().get_int();
                frame.operand_stack.push(Slot::Int(v2 + v1));
            }
            Instr::ISub => {
                let v2 = frame.operand_stack.pop().unwrap().get_int();
                let v1 = frame.operand_stack.pop().unwrap().get_int();
                frame.operand_stack.push(Slot::Int(v1 - v2));
            }
            Instr::IMul => {
                let v2 = frame.operand_stack.pop().unwrap().get_int();
                let v1 = frame.operand_stack.pop().unwrap().get_int();
                frame.operand_stack.push(Slot::Int(v1 * v2));
            }
            Instr::IDiv => {
                let v2 = frame.operand_stack.pop().unwrap().get_int();
                let v1 = frame.operand_stack.pop().unwrap().get_int();
                frame.operand_stack.push(Slot::Int(v1 / v2));
            }
            Instr::INeg => {
                let v = frame.operand_stack.pop().unwrap().get_int();
                frame.operand_stack.push(Slot::Int(-v));
            }
            Instr::IRem => {
                let v2 = frame.operand_stack.pop().unwrap().get_int();
                let v1 = frame.operand_stack.pop().unwrap().get_int();
                frame.operand_stack.push(Slot::Int(v1 % v2));
            }
            _ => todo!()
        }
    }
}

#[derive(Debug)]
pub struct Instructions(Vec<Instr>);

impl Instructions{
    pub fn new() -> Self{
        Instructions(Vec::new())
    }
    pub fn get_instr(&self,index:usize) -> Option<Instr> {
        self.0.get(index).cloned()
    }
    pub fn len(&self) -> usize{
        self.0.len()
    }
}

impl From<Vec<Instr>> for Instructions{
    fn from(s: Vec<Instr>) -> Self {
        Self(s)
    }
}

impl Into<Vec<Instr>> for Instructions{
    fn into(self) -> Vec<Instr> {
        self.0
    }
}

impl ops::Add for Instructions{
    type Output = Instructions;

    fn add(self, rhs: Self) -> Self::Output {
        let mut instrs = self.0;
        let mut other = rhs.0;
        instrs.append(&mut other);
        Instructions(instrs)
    }
}


impl Display for Instr{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self{
            Instr::IPush(v) => write!(f, "ipush {}", v),
            Instr::IAdd => write!(f,"iadd"),
            Instr::ISub => write!(f, "isub"),
            Instr::IMul => write!(f, "imul"),
            Instr::IDiv => write!(f, "idiv"),
            Instr::INeg => write!(f, "ineg"),
            Instr::IRem => write!(f, "irem"),
            Instr::Call(refer) => write!(f, "call {}", refer),
            Instr::Goto(offset) => {write!(f, "goto {}", offset)}
        }
    }
}

impl Display for Instructions{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for instr in &self.0{
            write!(f, "{}\n", instr)?
        }
        Ok(())
    }
}