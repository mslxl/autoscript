use std::cmp::Ordering;
use std::fmt::Debug;
use num_cmp::NumCmp;

#[derive(Clone, Debug, PartialEq)]
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

    #[inline]
    pub fn set_bool(&mut self, val: bool){
        match self{
            Slot::Bool(v) => *v = val,
            Slot::Null => {
                *self = Slot::Bool(val)
            }
            _ => panic!()
        }
    }

    #[inline]
    pub fn get_bool(&self) -> bool {
        if let Slot::Bool(value) = self {
            *value
        }else{
            panic!()
        }
    }
}


impl PartialOrd for Slot{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Slot::Int(i1), Slot::Int(i2)) => i1.partial_cmp(i2),
            (Slot::Int(v1), Slot::Float(v2)) => NumCmp::num_cmp(*v1,*v2),
            (Slot::Int(v1) , Slot::Char(v2)) => NumCmp::num_cmp(*v1, *v2 as u32),

            (Slot::Float(i1), Slot::Float(i2)) => i1.partial_cmp(i2),
            (Slot::Float(v1), Slot::Int(v2)) => NumCmp::num_cmp(*v1, *v2),
            (Slot::Float(v1), Slot::Char(v2)) => NumCmp::num_cmp(*v1, *v2 as u32),

            (Slot::Char(v1), Slot::Char(v2)) => v1.partial_cmp(v2),
            (Slot::Char(v1), Slot::Float(v2)) => NumCmp::num_cmp(*v1 as u32, *v2),
            (Slot::Char(v1), Slot::Int(v2)) => NumCmp::num_cmp(*v1 as u32, *v2),
            _ => None
        }
    }
}