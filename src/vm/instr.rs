use std::cmp::Ordering;
use std::fmt::{Display, Formatter, write};
use std::ops;
use std::rc::Rc;
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

    BPush(bool),
    BAnd,
    BOr,

    CmpEq,
    CmpNe,
    CmpLe,
    CmpLt,
    CmpGe,
    CmpGt,

    Store(usize),
    Load(usize),
    Pop,
    Call(String),
    Goto(i32),
    ReturnValue,
    Return,
    Nop,
}

impl Instr {
    pub fn execute(&self, frame: &mut Frame) {
        unsafe {
            println!("{}:\t{}", frame.thread.as_ref().unwrap().pc(), self);
        }

        match self {
            Instr::IPush(value) => {
                let slot = Slot::Int(*value as i64);
                frame.operand_stack.push(slot);
            }
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
                frame.operand_stack.push(Slot::Float(v1 + v2));
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
            Instr::Store(idx) => {
                let slot = frame.operand_stack.pop().unwrap();
                frame.local_vars.set(*idx, slot)
            }
            Instr::Load(idx) => {
                let slot = frame.local_vars.get(*idx).clone();
                frame.operand_stack.push(slot);
            }
            Instr::Call(fn_signature) => {
                unsafe {
                    let thread = frame.thread.as_mut().unwrap();
                    let vm = thread.vm.as_ref().unwrap();
                    //TODO
                    let fn_prototype = vm.module_man.get("hello").unwrap().get_function_prototype(fn_signature).unwrap();
                    let new_frame = thread.push_new_frame(fn_prototype.local_var_size, Rc::clone(&fn_prototype));
                    for i in 0..fn_prototype.arg_num {
                        let idx = fn_prototype.arg_num - i - 1;
                        let slot = frame.operand_stack.pop().unwrap();
                        new_frame.local_vars.set(idx, slot)
                    }
                }
            }
            Instr::ReturnValue => {
                unsafe {
                    let thread = frame.thread.as_mut().unwrap();
                    let value = frame.operand_stack.pop().unwrap();
                    thread.pop_frame();
                    thread.current_frame_mut().operand_stack.push(value);
                }
            }
            Instr::Return => {
                unsafe {
                    let thread = frame.thread.as_mut().unwrap();
                    let frame = thread.pop_frame().unwrap();

                    println!("Process finish. Local variable table:");
                    let table = frame.local_vars;
                    for i in 0..table.len() {
                        println!("{}\t: {:?}", i, table.get(i))
                    }
                }
            }
            Instr::BPush(b) => {
                frame.operand_stack.push(Slot::Bool(*b))
            }
            Instr::BAnd => {
                let v2 = frame.operand_stack.pop().unwrap().get_bool();
                let v1 = frame.operand_stack.pop().unwrap().get_bool();
                frame.operand_stack.push(Slot::Bool(v2 && v1));
            }
            Instr::BOr => {
                let v2 = frame.operand_stack.pop().unwrap().get_bool();
                let v1 = frame.operand_stack.pop().unwrap().get_bool();
                frame.operand_stack.push(Slot::Bool(v2 || v1));
            }
            Instr::CmpGt => {
                let v2 = frame.operand_stack.pop().unwrap();
                let v1 = frame.operand_stack.pop().unwrap();
                let result = v1 > v2;
                frame.operand_stack.push(Slot::Bool(result));
            }
            Instr::CmpGe => {
                let v2 = frame.operand_stack.pop().unwrap();
                let v1 = frame.operand_stack.pop().unwrap();
                let result = v1 >= v2;
                frame.operand_stack.push(Slot::Bool(result));
            }
            Instr::CmpLe => {
                let v2 = frame.operand_stack.pop().unwrap();
                let v1 = frame.operand_stack.pop().unwrap();
                let result = v1 <= v2;
                frame.operand_stack.push(Slot::Bool(result));
            }
            Instr::CmpLt => {
                let v2 = frame.operand_stack.pop().unwrap();
                let v1 = frame.operand_stack.pop().unwrap();
                let result = v1 < v2;
                frame.operand_stack.push(Slot::Bool(result));
            }
            Instr::CmpEq => {
                let v2 = frame.operand_stack.pop().unwrap();
                let v1 = frame.operand_stack.pop().unwrap();
                let result = v1 == v2;
                frame.operand_stack.push(Slot::Bool(result));
            }
            Instr::CmpNe => {
                let v2 = frame.operand_stack.pop().unwrap();
                let v1 = frame.operand_stack.pop().unwrap();
                let result = v1 != v2;
                frame.operand_stack.push(Slot::Bool(result));
            }
            Instr::Nop => {}
            _ => todo!("{}", self)
        }
    }
}

#[derive(Debug, Clone)]
pub struct Instructions(Vec<Instr>);

impl Instructions {
    pub fn new() -> Self {
        Instructions(Vec::new())
    }
    pub fn get_instr(&self, index: usize) -> Option<Instr> {
        self.0.get(index).cloned()
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl From<Vec<Instr>> for Instructions {
    fn from(s: Vec<Instr>) -> Self {
        Self(s)
    }
}

impl Into<Vec<Instr>> for Instructions {
    fn into(self) -> Vec<Instr> {
        self.0
    }
}

impl ops::Add for Instructions {
    type Output = Instructions;

    fn add(self, rhs: Self) -> Self::Output {
        let mut instrs = self.0;
        let mut other = rhs.0;
        instrs.append(&mut other);
        Instructions(instrs)
    }
}


impl Display for Instr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Instr::IPush(v) => write!(f, "ipush {}", v),
            Instr::IAdd => write!(f, "iadd"),
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
            Instr::BPush(b) => write!(f, "bpush {}", b),
            Instr::BAnd => write!(f, "band"),
            Instr::BOr => write!(f, "bor"),
            Instr::CmpEq => write!(f, "cmp_eq"),
            Instr::CmpNe => write!(f, "cmp_ne"),
            Instr::CmpGt => write!(f, "cmp_gt"),
            Instr::CmpGe => write!(f, "cmp_ge"),
            Instr::CmpLt => write!(f, "cmp_lt"),
            Instr::CmpLe => write!(f, "cmp_le"),
            Instr::ReturnValue => write!(f, "retv"),
            Instr::Return => write!(f, "ret"),
            Instr::Nop => write!(f, "nop"),
            Instr::Store(idx) => write!(f, "store {}", idx),
            Instr::Load(idx) => write!(f, "load {}", idx),
            Instr::Pop => write!(f, "pop"),
        }
    }
}

impl Display for Instructions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for instr in &self.0 {
            write!(f, "{}\n", instr)?
        }
        Ok(())
    }
}