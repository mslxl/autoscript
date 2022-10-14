use crate::vm::mem::ObjCore;

#[derive(Debug)]
pub struct ObjI32(pub i32);
impl ObjI32 {
    pub const NAME: &'static str = "prelude.i32";
}

impl ToString for ObjI32{
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

unsafe impl ObjCore for ObjI32 {
    fn name(&self) -> &str {
        Self::NAME
    }
}

#[derive(Debug)]
pub struct ObjStr(pub String);
impl ObjStr {
    pub const NAME: &'static str = "prelude.str";
}

impl ToString for ObjStr{
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

unsafe impl ObjCore for ObjStr {
    fn name(&self) -> &str {
        Self::NAME
    }
}