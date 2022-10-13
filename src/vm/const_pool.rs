use super::slot::Slot;

#[derive(Debug)]
pub struct ConstantPool(Vec<Slot>);

impl ConstantPool {

    pub fn get(&self, idx: usize) -> Option<Slot>{
        self.0.get(idx).map(Clone::clone)
    }
}

impl From<Vec<Slot>> for ConstantPool {
    fn from(vec: Vec<Slot>) -> Self {
        Self(vec)
    }
}