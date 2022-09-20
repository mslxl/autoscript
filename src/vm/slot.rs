use std::fmt::{Debug, Formatter, write};

#[derive(Clone, Debug)]
pub enum Slot{
    Null,
    Int(i64),
    Float(f64),
    Char(char),
    Bool(bool),
    Ref
}


// impl Debug for Slot{
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Slot::Null => write!(f, "null"),
//             _ => write!(f, "{:?}", self),
//         }
//     }
// }

impl Slot{
    #[inline]
    pub fn set_int(&mut self, val: i64){
        match self{
            Slot::Int(v) => *v = val,
            Slot::Null =>{
                *self = Slot::Int(val)
            }
            _ => panic!()
        }
    }

    #[inline]
    pub fn get_int(&self) -> i64 {
        if let Slot::Int(value) = self {
            *value
        }else{
            panic!()
        }
    }

    #[inline]
    pub fn set_float(&mut self, val: f64){
        match self{
            Slot::Float(v) => *v = val,
            Slot::Null => {
                *self = Slot::Float(val)
            }
            _ => panic!()
        }
    }

    #[inline]
    pub fn get_float(&self) -> f64 {
        if let Slot::Float(value) = self {
            *value
        }else{
            panic!()
        }
    }
}