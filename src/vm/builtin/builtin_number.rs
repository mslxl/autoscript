use crate::vm::obj::ObjInner;

#[derive(Debug)]
pub struct ObjI32(pub i32);
impl ObjI32 {
    pub const NAME: &'static str = "prelude.i32";
}

unsafe impl ObjInner for ObjI32 {
    fn name(&self) -> &str {
        Self::NAME
    }
}