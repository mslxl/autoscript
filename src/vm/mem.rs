use std::sync::RwLock;

#[derive(Debug)]
pub struct MemInner{

}
impl MemInner{
    fn new()->Self{
        Self{

        }
    }
}

#[derive(Debug)]
pub struct Mem(RwLock<MemInner>);

impl Mem{
    pub fn new() -> Self{
        Self(RwLock::new(MemInner::new()))
    }
}