use crate::vm::mem::ObjCore;


#[derive(Debug)]
pub struct ObjI32(pub i32);
impl ObjI32 {
    pub const NAME: &'static str = "prelude.i32";
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

unsafe impl ObjCore for ObjStr {
    fn name(&self) -> &str {
        Self::NAME
    }
}