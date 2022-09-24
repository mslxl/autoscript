use std::fmt::{Display, Formatter};
use std::ops;
use crate::vm::slot::Slot;
use crate::vm::thread::Frame;

#[derive(Clone, Debug)]
pub enum Instr {
    IPush(i64),
    IAdd,
    ISub,
    IMul,
    IDiv,
    INeg,
    IRem,

    I2F,
    F2I,

    FPush(f64),
    FAdd,
    FSub,
    FMul,
    FDiv,
    FNeg,
    FRem,

    Store(usize),
    Load(usize),
    Pop,
    Call(String),
    Goto(i32),
    ReturnValue,
    Return,
    Nop,
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

            Instr::I2F => {
                let v = frame.operand_stack.pop().unwrap().get_int();
                frame.operand_stack.push(Slot::Float(v as f64));
            }
            Instr::F2I => {
                let v = frame.operand_stack.pop().unwrap().get_float();
                frame.operand_stack.push(Slot::Int(v as i64));
            }
            Instr::FPush(value) => {
                frame.operand_stack.push(Slot::Float(*value));
            }
            Instr::FAdd => {
                let v2 = frame.operand_stack.pop().unwrap().get_float();
                let v1 = frame.operand_stack.pop().unwrap().get_float();
                frame.operand_stack.push(Slot::Float(v1 % v2));
            }
            Instr::FSub => {
                let v2 = frame.operand_stack.pop().unwrap().get_float();
                let v1 = frame.operand_stack.pop().unwrap().get_float();
                frame.operand_stack.push(Slot::Float(v1 - v2));
            }
            Instr::FMul => {
                let v2 = frame.operand_stack.pop().unwrap().get_float();
                let v1 = frame.operand_stack.pop().unwrap().get_float();
                frame.operand_stack.push(Slot::Float(v1 * v2));
            }
            Instr::FDiv => {
                let v2 = frame.operand_stack.pop().unwrap().get_float();
                let v1 = frame.operand_stack.pop().unwrap().get_float();
                frame.operand_stack.push(Slot::Float(v1 / v2));
            }
            Instr::FNeg => {
                let v = frame.operand_stack.pop().unwrap().get_float();
                frame.operand_stack.push(Slot::Float(-v));
            }
            Instr::FRem => {
                let v2 = frame.operand_stack.pop().unwrap().get_float();
                let v1 = frame.operand_stack.pop().unwrap().get_float();
                frame.operand_stack.push(Slot::Float(v1 % v2));
            }
            Instr::ReturnValue => {
                println!("Top: {:?}", frame.operand_stack.first());
                todo!()
            }
            Instr::Return => {
                println!("Top: {:?}", frame.operand_stack.first());
                todo!()
            }
            Instr::Store(idx) => {
                let slot = frame.operand_stack.pop().unwrap();
                frame.local_vars.set(*idx, slot)
            }
            Instr::Load(idx) => {
                let slot = frame.local_vars.get(*idx).clone();
                frame.operand_stack.push(slot);
            }
            Instr::Nop => {}
            _ => todo!("{}",self)
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
            Instr::Goto(offset) => write!(f, "goto {}", offset),
            Instr::I2F => write!(f, "i2f"),
            Instr::F2I => write!(f, "f2i"),
            Instr::FPush(value) => write!(f, "fpush {}", value),
            Instr::FAdd => write!(f, "fadd"),
            Instr::FSub => write!(f, "fsub"),
            Instr::FMul => write!(f, "fmul"),
            Instr::FDiv => write!(f, "fdiv"),
            Instr::FNeg => write!(f, "fneg"),
            Instr::FRem => write!(f, "frem"),
            Instr::ReturnValue => write!(f, "retv"),
            Instr::Return => write!(f, "ret"),
            Instr::Nop => write!(f, "nop"),
            Instr::Store(idx) => write!(f, "store {}", idx),
            Instr::Load(idx) => write!(f, "load {}", idx),
            Instr::Pop => write!(f, "pop"),
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